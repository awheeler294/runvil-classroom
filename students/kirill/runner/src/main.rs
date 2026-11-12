mod nash;

use harness_gateway_client::{GatewayBroker, GatewayEndpoint, SecretString};
use runvil::{
    BoxFuture, Error, ErrorCode, Harness, Host, RunSetup,
    effect::Round,
    harness::{InferenceBroker, MemoryRecorder, Packs, Timer},
    model::{
        Completion, CompletionOptions, Message, ModelBinding, ModelCatalog, ToolSchema,
        make_completion_error, phrase,
    },
    plugin::{HostServices, PackRegistry},
};
use serde_json::{Value, json};
use std::{
    env, error, fs, io,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
    time::Duration,
};

type Failure = Box<dyn error::Error>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);
    let (Some(prompt), Some(outputs)) = (arguments.next(), arguments.next()) else {
        eprintln!("usage: nash-runner <prompt.md> <output-dir> [args...]");
        return ExitCode::from(2);
    };
    let args = arguments.collect::<Vec<_>>().join(" ");
    let server = env::var("NASH_HOST_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_owned());
    let written = match broker() {
        Ok(broker) => {
            run(
                Path::new(&prompt),
                Path::new(&outputs),
                args,
                broker,
                &server,
            )
            .await
        }
        Err(failure) => Err(failure),
    };
    match written {
        Ok(path) => {
            eprintln!("wrote {}", path.display());
            ExitCode::SUCCESS
        }
        Err(failure) => {
            eprintln!("error: {failure}");
            ExitCode::FAILURE
        }
    }
}

async fn run(
    prompt: &Path,
    outputs: &Path,
    args: String,
    broker: Arc<dyn InferenceBroker>,
    server: &str,
) -> Result<PathBuf, Failure> {
    let name = prompt
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("the prompt path names no file")?;
    let stem = Path::new(name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(name);
    let folder = prompt.parent().map(Path::to_owned).unwrap_or_default();

    let mut registry = PackRegistry::new();
    plugin_std::install(&mut registry, Value::Null, &HostServices::new())?;
    nash::install(&mut registry, server)?;
    let host = Arc::new(Host {
        packs: Packs::new(registry),
        prompts: Arc::new(move |name: &str| find(&folder, name)),
        timer: Arc::new(Clock),
    });
    let parsed = host.parse(name)?;
    let declared = parsed
        .frontmatter()
        .output()
        .map(|output| output.path().to_owned());

    let recorder = Arc::new(MemoryRecorder::new());
    let setup = RunSetup {
        args,
        input_text: env::var_os("RUNNER_INPUT")
            .map(fs::read_to_string)
            .transpose()?,
        selected_model: env::var("RUNNER_MODEL").ok(),
        ..RunSetup::new(recorder.clone(), broker)
    };
    let report = Harness::new(host, parsed, setup).run().await?;

    fs::create_dir_all(outputs)?;
    let trace = recorder
        .records(report.run_id)
        .into_iter()
        .map(|record| {
            let line = json!({
                "task": record.task_id,
                "seq": record.task_seq,
                "kind": record.kind.as_str(),
                "effect": record.effect_id,
                "payload": record.payload,
            });
            format!("{line}\n")
        })
        .collect::<String>();
    fs::write(outputs.join(format!("{stem}.trace.jsonl")), trace)?;

    let text = report.result?;
    println!("{text}");
    let (document, extension) = match &declared {
        Some(path) => (
            report.output?,
            Path::new(path).extension().and_then(|kind| kind.to_str()),
        ),
        None => (text, None),
    };
    let written = outputs.join(format!("{stem}.{}", extension.unwrap_or("md")));
    fs::write(&written, document)?;
    Ok(written)
}

fn broker() -> Result<Arc<dyn InferenceBroker>, Failure> {
    let Ok(url) = env::var("RUNVIL_GATEWAY_URL") else {
        eprintln!("RUNVIL_GATEWAY_URL is not set: running offline, every model round fails");
        return Ok(Arc::new(Offline));
    };
    let key = env::var("RUNVIL_GATEWAY_API_KEY")
        .map_err(|_| "RUNVIL_GATEWAY_URL is set, but RUNVIL_GATEWAY_API_KEY is not")?;
    let broker = GatewayBroker::new(GatewayEndpoint::new(&url)?, SecretString::new(key)?);
    Ok(Arc::new(broker))
}

fn find(folder: &Path, name: &str) -> Result<String, Error> {
    let path = folder.join(name);
    fs::read_to_string(&path).map_err(|failure| {
        let code = if failure.kind() == io::ErrorKind::NotFound {
            ErrorCode::NotFound
        } else {
            ErrorCode::Io
        };
        Error::new(code, format!("{}: {failure}", path.display()), code.prose())
    })
}

struct Offline;

impl InferenceBroker for Offline {
    fn models(&self) -> BoxFuture<'static, Result<ModelCatalog, Error>> {
        Box::pin(async { Ok(ModelCatalog::empty()) })
    }

    fn chat(
        &self,
        _binding: ModelBinding,
        _messages: Vec<Message>,
        _tools: Vec<ToolSchema>,
        _options: CompletionOptions,
        _round: Round,
    ) -> BoxFuture<'static, Result<Box<Completion>, Error>> {
        let code = ErrorCode::Unavailable;
        Box::pin(async move { Err(make_completion_error(code, phrase(code))) })
    }
}

struct Clock;

impl Timer for Clock {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio::time::sleep(duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        process, str, thread,
    };

    #[tokio::test]
    async fn a_model_free_prompt_leaves_its_result_and_trace() {
        let folder = env::temp_dir().join(format!("nash-runner-{}", process::id()));
        let outputs = folder.join("outputs");
        fs::create_dir_all(&folder).unwrap();
        let prompt = folder.join("2026.greet.md");
        let source = "---\nname: greet\ndescription: Greets the argument\nrunvil: 0\nuses:\n  - std\n---\n\n# Greet\n\n## Answer\n\n```lua\nreturn 'Hello, ' .. args .. '.'\n```\n";
        fs::write(&prompt, source).unwrap();

        let written = run(
            &prompt,
            &outputs,
            "desk".to_owned(),
            Arc::new(Offline),
            "http://127.0.0.1:3000",
        )
        .await
        .unwrap();

        assert_eq!(written, outputs.join("2026.greet.md"));
        assert_eq!(fs::read_to_string(&written).unwrap(), "Hello, desk.");
        let trace = fs::read_to_string(outputs.join("2026.greet.trace.jsonl")).unwrap();
        let last = trace
            .lines()
            .last()
            .map(|line| serde_json::from_str::<Value>(line).unwrap()["payload"]["kind"].clone());
        assert_eq!(last, Some(json!("run_succeeded")));
        fs::remove_dir_all(folder).unwrap();
    }

    #[tokio::test]
    async fn a_prompt_reaches_the_nash_host_through_the_request_tool() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let server = format!("http://{}/", listener.local_addr().unwrap());
        let seen = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = String::new();
            let mut chunk = [0; 4096];
            loop {
                let read = stream.read(&mut chunk).unwrap();
                request.push_str(str::from_utf8(&chunk[..read]).unwrap());
                if read == 0 || complete(&request) {
                    break;
                }
            }
            stream
                .write_all(b"HTTP/1.1 204 No Content\r\nconnection: close\r\n\r\n")
                .unwrap();
            request
        });
        let folder = env::temp_dir().join(format!("nash-runner-reach-{}", process::id()));
        let outputs = folder.join("outputs");
        fs::create_dir_all(&folder).unwrap();
        let prompt = folder.join("reach.md");
        let source = "---\nname: reach\ndescription: Sends one request to the Nash host\nrunvil: 0\nuses:\n  - kirill/nash\n---\n\n# Reach\n\n## Send\n\n```lua\nreturn tools.call('nash/request', { method = 'PUT', path = '/api/players/Prisoner%20A', body = '{\"summary\":\"x\"}' })\n```\n";
        fs::write(&prompt, source).unwrap();

        let written = run(&prompt, &outputs, String::new(), Arc::new(Offline), &server)
            .await
            .unwrap();

        let reply = fs::read_to_string(&written).unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&reply).unwrap(),
            json!({ "status": 204, "body": "" })
        );
        let request = seen.join().unwrap();
        let (head, body) = request.split_once("\r\n\r\n").unwrap();
        let content_type = head
            .lines()
            .filter(|line| line.to_ascii_lowercase().starts_with("content-type:"))
            .collect::<Vec<_>>();
        assert_eq!(
            (head.lines().next(), content_type, body),
            (
                Some("PUT /api/players/Prisoner%20A HTTP/1.1"),
                vec!["content-type: application/json"],
                "{\"summary\":\"x\"}"
            )
        );
        fs::remove_dir_all(folder).unwrap();
    }

    fn complete(request: &str) -> bool {
        request.split_once("\r\n\r\n").is_some_and(|(head, body)| {
            let length = head.lines().find_map(|line| {
                line.to_ascii_lowercase()
                    .strip_prefix("content-length:")?
                    .trim()
                    .parse::<usize>()
                    .ok()
            });
            body.len() >= length.unwrap_or(0)
        })
    }
}
