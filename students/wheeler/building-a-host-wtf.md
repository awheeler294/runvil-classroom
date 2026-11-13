- I want to build a `runvil::Harness` 
    - First I need a `runvil::Host`
        - There is no constructor
        - `What every run of one Host shares: its installed Packs, how it finds prompts, and its timer. A Host builds one and shares it behind an `Arc`.` A Host builds a Host? What do `Arc`s have to do with anything?
        - I need a `runvil::harness::Packs`
            - I need a `runvil::plugin::PackRegistry`
        - I need a type that implements the `PromptFinder` trait
            - The hell is this gibberish? That second scentance not even prooper English `Finds a prompt's source by its name, such as `research.md`. The name is the prompt's identity: [`Prompt::file`](crate::Prompt::file) and every failure use it.`
            - So `PromptFinder` has one method `find` that takes `name: &str` and returns a `String`. I still don't know what it's supposed to do or what information to put in that `string`. My first guess would be a file path to the prompt, because that's what a finder would find, right?
            - BTW at this point we're implementing a language feature at the host level: The engine is parsing a prompt and finds a sub-prompt that needs to be parsed, so it calls `PromptFinder::find` with the name of that prompt. Since `find` is defined way up in the host, I as the host developer have to decide where those sub-prompts are located. That should be decided at the language level, not left as a host implementation detail.
        - I need a type that implements the `Timer` trait.
            - Surly this should be provided by the harness, not the host. (And don't call me Shirley!)
            - What is a `BoxFuture`? oh, it's a `Pin<Box<dyn Future<Output = T> + Send + 'a>>`, that totally clears it up! If this is going to be part of our public API, it needs to be easier to construct. But (see above), I don't really think it should be part of our public API.

- `runvil::Error::new(code, format!("{}: {err}", path.display()), code.prose())` why are we passing `code` and `code.prose()` to the `Error` constructor? the `Error` already has `code`, it can call `prose` on it any time it wants.




