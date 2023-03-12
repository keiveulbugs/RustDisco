mod commands;

use clap::Parser;
use clap::builder::Str;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::{Write, Seek};
use std::fs;
use std::io::SeekFrom;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the bot to create
   #[arg(short, long)]
   n: String,
}

fn main() {
   let args = Args::parse();


   // Here starts the creation of a new library.
   println!("\nLet the Disco start 🎉!\n");

   // Runs ```cargo new``` to create a new directory.
Command::new("cargo")
      .arg("new")
      .arg(args.n.clone())
      .status()
      .expect("Running cargo new failed");

   println!("\nHi {}! Welcome to the party!\n", args.n);

   // Update the Cargo.toml to include Poise and Tokio
   let mut fileref = OpenOptions::new().append(true).open(format!(r"{}\Cargo.toml", args.n)).expect("Unable to open file");   
   fileref.write_all(r#"poise = { version = "0.4\5.2", features = ["cache"] }"#.as_bytes()).expect("Failed to add Poise to Cargo.toml");
   fileref.write_all("\n".as_bytes()).expect("Failed to start a new line after Poise in Cargo.toml");   
   fileref.write_all(r#"tokio = { version = "1", features = ["macros", "rt-multi-thread"] }"#.as_bytes()).expect("Failed to add Tokio to Cargo.toml");
   fileref.write_all("\n".as_bytes()).expect("Failed to start a new line after Poise in Cargo.toml");   
   fileref.write_all(r#"serenity = { version = "0.11.5", default-features = false, features = ["client", "gateway", "rustls_backend", "model"] }"#.as_bytes()).expect("Failed to add Tokio to Cargo.toml");
   fileref.write_all("\n".as_bytes()).expect("Failed to start a new line after Poise in Cargo.toml");   
   fileref.write_all(r#"dotenv_codegen = "0.15.0""#.as_bytes()).expect("Failed to add dotenv_codegen to Cargo.toml");
   println!("Succesfully added all dependencies to Cargo.toml!");

   // Create a commands directory
   fs::create_dir(format!(r"{}\src\commands", args.n)).expect("Failed creating the commands directory");
   println!("Succesfully created commands directory!");


   // Create mod.rs
   let mut file = fs::File::create(format!(r"{}\src\commands\mod.rs", args.n)).expect("creating command file failed");

   // Fill main.rs
   let mainbytes = commands::setupmain::setupmainfn();
   let mut mainfile = OpenOptions::new().write(true).truncate(true).open(format!(r"{}\src\main.rs", args.n)).expect("Unable to open main.rs");
   mainfile.write_all(mainbytes.as_bytes()).expect("Couldn't write to main.rs");
   println!("Succesfully created the main.rs file!");

   // Create the help command
   let helpbytes = std::fs::read_to_string(r"src\commands\default_commands\help.rs").expect("Couldn't read help command file");
   let mut helpfile = fs::File::create(format!(r"{}\src\commands\help.rs", args.n)).expect("couldn't create help command file");
   helpfile.write_all(helpbytes.as_bytes()).expect("Couldn't create the help command");
   file.write_all(b"pub mod help;\n").expect("Failed to update mod");
   println!("Succesfully created the help command!");



   commands::setupcommand::setupcommand("examplecommand".to_string(), args.n.clone());
   println!("Succesfully created an examplecommand!");

   let mut envfile = fs::File::create(format!(r"{}\.env", args.n)).expect("Couldn't create a .env file");
   envfile.write_all("DISCORD_TOKEN=".as_bytes()).expect("Couldn't write to .env file");
   println!("Succesfully added a .env file!");


}



