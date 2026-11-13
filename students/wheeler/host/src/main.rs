use std::{env, fs, io, path::PathBuf, sync::Arc};

use runvil::{
    Harness, Host,
    harness::{Packs, Timer},
    plugin::PackRegistry,
};

fn main() {
    println!("Hello, world!");

    let mut arg_iter = env::args().skip(1);

    let (Some(prompt_arg), Some(output_arg)) = (arg_iter.next(), arg_iter.next()) else {
        eprintln!("usage: host <prompt> <output-dir>");
        std::process::exit(2);
    };

    let prompt_path = PathBuf::from(&prompt_arg);
    let output_path = PathBuf::from(&output_arg);

    if !prompt_path.is_file() {
        eprintln!("Cannot find prompt file {prompt_path:?}");
        std::process::exit(3);
    }

    let prompt_dir = {
        let Some(prompt_dir) = prompt_path.parent() else {
            eprintln!("Cannot find parent directory of {prompt_path:?}");
            std::process::exit(4);
        };

        PathBuf::from(prompt_dir)
    };

    dbg!(&prompt_path);
    dbg!(&output_path);

    let find = move |name: &str| -> Result<String, runvil::Error> {
        let path = prompt_dir.join(name);

        fs::read_to_string(&path).map_err(|err| {
            let code = if err.kind() == io::ErrorKind::NotFound {
                runvil::ErrorCode::NotFound
            } else {
                runvil::ErrorCode::Io
            };

            runvil::Error::new(code, format!("{}: {err}", path.display()), code.prose())
        })
    };

    let registry = PackRegistry::new();

    let packs = Packs::new(registry);

    let host = Host {
        packs,
        prompts: Arc::new(find),
        timer: Arc::new(HostTimer),
    };

    // let harness = Harness::new(host, prompt, setup)
}

struct HostTimer;

impl Timer for HostTimer {
    fn sleep(&self, duration: std::time::Duration) -> runvil::BoxFuture<'static, ()> {
        Box::pin(async move { std::thread::sleep(duration) })
    }
}
