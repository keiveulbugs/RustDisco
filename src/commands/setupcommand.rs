use std::fs;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::io::{Seek, Write};

use crate::Asset;

pub fn setupcommand(name: String, pathname: String, commandtype: u64) {
    // inserting name of command
    let mut mainfilebuffer = fs::read_to_string(format!(r"{}\src\main.rs", pathname))
        .expect("Failed to read main.rs to String");
    let position = mainfilebuffer
        .find("commands::help")
        .expect("Failed to find the position of `commands::help`");
    mainfilebuffer.insert_str(
        position,
        format!("        commands::{}::{}(),\n", name, name).as_str(),
    );

    let mut mainfile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!(r"{}\src\main.rs", pathname))
        .expect("Unable to open main.rs");

    mainfile
        .seek(SeekFrom::Start(0))
        .expect("Failed to put cursor writing at 0 in the main.rs.");
    mainfile
        .write_all(mainfilebuffer.as_bytes())
        .expect("Couldn't add example command to main.rs.");

    // Create command file
    let mut file = fs::File::create(format!(r"{}\src\commands\{}.rs", pathname, name))
        .expect("Creating command file failed.");

    // updating mod.rs file
    let mut modfile = OpenOptions::new()
        .append(true)
        .truncate(false)
        .open(format!(r"{}\src\commands\mod.rs", pathname))
        .expect("Unable to open mod.rs for new command.");
    modfile
        .write_all(format!("pub mod {};\n", name).as_bytes())
        .expect("Failed to add the new command file");

    // Create new command in command file
    let importexample = if commandtype == 0 {
        Asset::get("def_com/default.rs").expect("Couldn't fetch default command file")
    } else if commandtype == 1 {
        Asset::get("def_com/example.rs").expect("Couldn't fetch example command file")
    } else if commandtype == 2 {
        Asset::get("def_com/purge.rs").expect("Couldn't fetch purge command file")
    } else if commandtype == 3 {
        Asset::get("def_com/status.rs").expect("Couldn't fetch status command file")
    } else if commandtype == 4 {
        Asset::get("def_com/registration.rs").expect("Couldn't fetch registration command file")
    } else {
        Asset::get("def_com/default.rs").expect("Couldn't fetch default command file")
    };

    let commandstring = std::str::from_utf8(importexample.data.as_ref())
        .expect("Couldn't convert command file to bytes");
    let newstring = commandstring.replace("examplecommand", name.as_str());

    file.write_all(newstring.as_bytes())
        .expect("Failed to write the new example command to the command file.");
}
