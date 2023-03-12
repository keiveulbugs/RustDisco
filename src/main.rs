mod commands;

use clap::Parser;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
use rust_embed::RustEmbed;


/// A program to generate templats for Discord bots using Serenity and Poise
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the bot to create
   #[arg(short, long)]
   name: String,


   /// Deploy a basic command
   #[arg(short, long)]
   default: Option<String>,

   /// Deploy an example command
   #[arg(short, long)]
   example: Option<String>,
}


// including files
#[derive(RustEmbed)]
#[folder = "src/commands/default_commands"]
#[prefix = "def_com/"]
pub struct Asset;



fn main() {
   let args = Args::parse();


   // Here starts the creation of a new library.
   println!("\nLet the Disco start 🎉!\n");

   // Runs ```cargo new``` to create a new directory.
Command::new("cargo")
      .arg("new")
      .arg(args.name.clone())
      .status()
      .expect("Running cargo new failed");

   println!("\nHi {}! Welcome to the party!\n", args.name);

   // Update the Cargo.toml to include Poise and Tokio
   let mut fileref = OpenOptions::new().append(true).open(format!(r"{}\Cargo.toml", args.name)).expect("Unable to open file");
   fileref.write_all(
   r#"poise = { version = "0.4\5.2", features = ["cache"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serenity = { version = "0.11.5", default-features = false, features = ["client", "gateway", "rustls_backend", "model"] }
dotenv_codegen = "0.15.0"
   "#.as_bytes()).expect("Failed to add dependencies");
   println!("Succesfully added all dependencies to Cargo.toml!");

   // Create a commands directory
   fs::create_dir(format!(r"{}\src\commands", args.name)).expect("Failed creating the commands directory");
   println!("Succesfully created commands directory!");


   // Create mod.rs
   let mut file = fs::File::create(format!(r"{}\src\commands\mod.rs", args.name)).expect("creating command file failed");

   // Fill main.rs
   let mainimport = Asset::get("def_com/main.rs").expect("Couldn't fetch main.rs file");
   let mainbytes = std::str::from_utf8(mainimport.data.as_ref()).expect("Couldn't convert main.rs to bytes");
   let mut mainfile = OpenOptions::new().write(true).truncate(true).open(format!(r"{}\src\main.rs", args.name)).expect("Unable to open main.rs");
   mainfile.write_all(mainbytes.as_bytes()).expect("Couldn't write to main.rs");
   println!("Succesfully created the main.rs file!");

   // Create the help command
   let helpimport = Asset::get("def_com/help.rs").expect("Couldn't fetch help command file");
   let helpbytes = std::str::from_utf8(helpimport.data.as_ref()).expect("Couldn't convert help.rs to bytes");
   let mut helpfile = fs::File::create(format!(r"{}\src\commands\help.rs", args.name)).expect("couldn't create help command file");
   helpfile.write_all(helpbytes.as_bytes()).expect("Couldn't create the help command");
   file.write_all(b"pub mod help;\n").expect("Failed to update mod");
   println!("Succesfully created the help command!");

   // Create .env file
   let mut envfile = fs::File::create(format!(r"{}\.env", args.name)).expect("Couldn't create a .env file");
   envfile.write_all("DISCORD_TOKEN=".as_bytes()).expect("Couldn't write to .env file");
   println!("Succesfully added a .env file!");

   let mut commandtype = 0;

   if args.default.is_some() {
      commandtype = 0;
      let name = args.default.unwrap_or("template".to_string());
      commands::setupcommand::setupcommand(name.clone(), args.name.clone(), commandtype);
      println!("Succesfully created a command with the name '{}'!", name);
   }

   if args.example.is_some() {
      commandtype = 1;
      let name = args.example.unwrap_or("template".to_string());
      commands::setupcommand::setupcommand(name.clone(), args.name.clone(), commandtype);
      println!("Succesfully created a command with the name '{}'!", name);
   }
   


}



