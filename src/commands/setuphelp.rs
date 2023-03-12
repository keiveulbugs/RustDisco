pub fn setuphelpfn() -> String {
    let setuphelpstring = std::fs::read_to_string(r"src\commands\default_commands\help.rs").expect("Couldn't read help command file");
    return setuphelpstring.to_string();
}