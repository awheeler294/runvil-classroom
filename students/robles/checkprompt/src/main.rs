// checkprompt prompt.md

// Code 0: Success
// Code 9: Bad command line
// Code 8: Cannot read file path
// Code 7: Cannot parse prompt
// Code 6: Cannot run prompt
// Usage: checkprompt <prompt.md> [--run] [--args <JSON>]

use std::env;
use std::fs;
use std::process::ExitCode;
use std::path::Path;
use runvil::{BoxFuture, Harness, Host, RunSetup};

use runvil::harness::FindFilePrompt;
use std::sync::Arc;
use std::time::Duration;

use runvil::harness::{MemoryRecorder, Packs, Timer};

use harness_gateway_client::{GatewayBroker, GatewayEndpoint, SecretString};
use runvil::plugin::PackRegistry;

const BANNER: &str = r#"
       ██████╗██╗  ██╗███████╗ ██████╗██╗  ██╗
      ██╔════╝██║  ██║██╔════╝██╔════╝██║ ██╔╝
      ██║     ███████║█████╗  ██║     █████╔╝
      ██║     ██╔══██║██╔══╝  ██║     ██╔═██╗
      ╚██████╗██║  ██║███████╗╚██████╗██║  ██╗
       ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝
██████╗ ██████╗  ██████╗ ███╗   ███╗██████╗ ████████╗
██╔══██╗██╔══██╗██╔═══██╗████╗ ████║██╔══██╗╚══██╔══╝
██████╔╝██████╔╝██║   ██║██╔████╔██║██████╔╝   ██║
██╔═══╝ ██╔══██╗██║   ██║██║╚██╔╝██║██╔═══╝    ██║
██║     ██║  ██║╚██████╔╝██║ ╚═╝ ██║██║        ██║
╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝        ╚═╝

─────────────────────────────────────────────────────
      check a Runvil prompt before you run it
─────────────────────────────────────────────────────
"#;

enum CheckError {
    NoArgumentGiven,
    ArgsValueMissing,
    CannotParsePrompt(String),
    CannotReadFilePath(String),
    CannotRunPrompt(String),
    
}

struct Clock;

impl Timer for Clock {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio::time::sleep(duration))
    }
}

impl CheckError {
    fn error_code(&self) -> ExitCode {
        match self {
            Self::NoArgumentGiven => ExitCode::from(9),
            Self::ArgsValueMissing => ExitCode::from(9),
            Self::CannotReadFilePath(_) => ExitCode::from(8),
            Self::CannotParsePrompt(_) => ExitCode::from(7),
            Self::CannotRunPrompt(_) => ExitCode::from(6),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::CannotReadFilePath(msg) => msg.clone(),
            Self::CannotParsePrompt(msg) => msg.clone(),
            Self::NoArgumentGiven => "No argument given".to_string(),
            Self::ArgsValueMissing => "The value of Args is missing".to_string(),
            Self::CannotRunPrompt(msg) => msg.clone(),
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {

    let cli_args: Vec<String> = env::args().collect();
    //println!("{}",cli_args[0])
    let executable: &String = &cli_args[0];
    let program = Path::new(executable)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("checkprompt");

    if cli_args.len() == 1 {
        println!("{BANNER}");
        println!("  usage: {program} <prompt.md> [--run] [--args <JSON>]");
        return ExitCode::SUCCESS;
    }

    let mut run = false;
    let mut path: Option<&String> = None;
    let mut prompt_args = "{}".to_string();

    let mut i = 1;                          
    while i < cli_args.len() {
        let word = &cli_args[i];
        if word == "--run" {
            run = true;                            
        } else if word == "--args" {
            i+=1;
            let Some(value) = cli_args.get(i) else {
                let err = CheckError::ArgsValueMissing;
                eprintln!("{program}: {}", err.message());
                return err.error_code();
            };
            prompt_args = value.clone();
          
        } else {
            path = Some(word);                   
        }
        i += 1;
    }

    let Some(path) = path else {
        let no_arg = CheckError::NoArgumentGiven;
        eprintln!("{program}: {}", no_arg.message());
        eprintln!("Usage: {program} <prompt.md> [--run] [--args <JSON>]");

        return no_arg.error_code();
    };

    match check(path, run, &prompt_args).await {
        Ok(()) => {
            println!("Success!");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{program}: {}", err.message());
            eprintln!("Usage: {program} <prompt.md> [--run] [--args <JSON>]");
            err.error_code()
        }
    }
}

async fn check(path: &str, run: bool, prompt_args: &str) -> Result<(), CheckError> {
    let text = fs::read_to_string(path);

    let prompt_file = match text {
        Ok(text) => text,
        Err(e) => {
            return Err(CheckError::CannotReadFilePath(format!(
                "Cannot read file {path}, {e}"
            )));
        }
    };

    let bytes = prompt_file.len();
    println!("Bytes read: {bytes}");

    let host = Arc::new(Host {
        packs: Packs::new(PackRegistry::new()),
        prompts: Arc::new(FindFilePrompt::new(path, path)),
        timer: Arc::new(Clock),
    });

    let prompt = match host.parse(path) {
        Ok(p) => p,
        Err(e) => {
            return Err(CheckError::CannotParsePrompt(format!(
                "Cannot parse prompt {path}, {e}"
            )));
        }
    };

    if !run {
        return Ok(());
    }

    let url = match env::var("RUNVIL_GATEWAY_URL") {
        Ok(value) => value,
        Err(e) => {
            return Err(CheckError::CannotRunPrompt(format!(
                "RUNVIL_GATEWAY_URL: {e}"
            )));
        }
    };

    let key = match env::var("RUNVIL_GATEWAY_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            return Err(CheckError::CannotRunPrompt(format!(
                "RUNVIL_GATEWAY_API_KEY: {e}"
            )));
        }
    };

    let endpoint = match GatewayEndpoint::new(&url) {
        Ok(endpoint) => endpoint,
        Err(e) => {
            return Err(CheckError::CannotRunPrompt(format!(
                "The RUNVIL Gateway Endpoint is improperly configured: {e}"
            )));
        }
    };

    let secret = match SecretString::new(key) {
        Ok(secret) => secret,
        Err(e) => {
            return Err(CheckError::CannotRunPrompt(format!(
                "The RUNVIL Secret Key is Empty: {e}"
            )));
        }
    };

    let broker = Arc::new(GatewayBroker::new(endpoint, secret));

    let recorder = Arc::new(MemoryRecorder::new());

    let setup = RunSetup {
        args: prompt_args.to_owned(),
        selected_model: Some("gpt-5-mini".to_owned()),
        ..RunSetup::new(recorder, broker)
    };

    let report = match Harness::new(host, prompt, setup).run().await {
        Ok(report) => report,
        Err(e) => {
            return Err(CheckError::CannotRunPrompt(format!(
                "Cannot initialize Harness: {e}"
            )));
        }
    };

    match report.result {
        Ok(text) => println!("{text}"),
        Err(e) => {
            return Err(CheckError::CannotRunPrompt(format!(
                "Run failed: {e}"
            )));
        }
    }

    Ok(())
}
