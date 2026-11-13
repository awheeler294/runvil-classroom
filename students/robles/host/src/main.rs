// Set up Packs
use std::sync::Arc;
use std::time::Duration;

use runvil::effect::Round;
use runvil::harness::{InferenceBroker, MemoryRecorder, Packs, Timer};
use runvil::model::{
    Completion, CompletionOptions, Message, ModelBinding, ModelCatalog, ToolSchema,
    make_completion_error, phrase,
};
use runvil::plugin::PackRegistry;
use runvil::{BoxFuture, Error, ErrorCode, Harness, Host, RunSetup};

struct Clock;
// implement Clock via Timer interface
impl Timer for Clock {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio::time::sleep(duration))
    }
}
/// Lists no model and refuses every round, since the prompt asks no model.
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

// allow async main function
#[tokio::main(flavor = "current_thread")]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    // set absolute path of prompt
    let path = "D:/runvil/repos/runvil/prompts/hello.md";

    // parse file to string
    let text = std::fs::read_to_string(path).expect("could not read the file");

    // find prompt from name
    let finder = move |_name: &str| -> Result<String, runvil::Error> { Ok(text.clone()) };
    
    // create new recorder object
    let recorder = Arc::new(MemoryRecorder::new());
    
    // create new RunSetup object
    let setup = RunSetup::new(recorder.clone(), Arc::new(Offline));
  
    // create new host object
    let host = Arc::new(Host {
        packs: Packs::new(PackRegistry::new()),
        prompts: Arc::new(finder),
        timer: Arc::new(Clock),
    });
    
    // parse prompt
    let prompt = host.parse("hello.md")?;

    // create new harness object
    let report = Harness::new(host, prompt, setup).run().await?;

    match report.result { Ok(text) => println!("parsed: {text}"), Err(e) => println!("error: {e}")};
    Ok(())
}

