fn main() {
    let path = "D:/runvil/repos/runvil/prompts/hello.md";
    let text = std::fs::read_to_string(path).expect("could not read the file");
    println!("the answer is {}", text.len());


    let finder = |_name: &str| -> Result<String,runvil::Error> { Ok(text.clone()) };

    let found = runvil::Prompt::parse(&finder, "hello.md"); 

    match found {
        Ok(_) => println!("parsed"),
        Err(problem) => println!("error:, {problem}"),
    }

}