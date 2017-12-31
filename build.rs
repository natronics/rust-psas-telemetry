/*
This build file will look for TOML files in the directory `./definitions/` and try
and read them. It will then write a Rust source file for each one that will
define structs and methods for each message definition.

It also writes the main `messages` module.
*/
#[macro_use] extern crate tera;
#[macro_use] extern crate lazy_static;
extern crate toml;
extern crate glob;

use tera::{Tera, Context};
use toml::de::Error as TError;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::Read;
use glob::glob;


// Compile any Rust templates in the template directory
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = compile_templates!("templates/*.rs");
        tera
    };
}


fn main() {
    // We'll write all our code to the `messages` module
    let out_dir = "src/messages/";

    // All the message definitions (read from TOML files) will be stored in a list here:
    let mut message_definitions = vec![];

    // Any TOML file in the directory `definitions` is expected to be a definition of a
    // PSAS Telemetry message definition.
    // Loop through all of them:
    for entry in glob("definitions/*.toml").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // We found a definition! Open and parse the TOML
                let data = open_definition(path).unwrap();
                // Store it in our list of definitions
                message_definitions.push(data);
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }

    // Now we have read all of our TOML definitions into memory. We pass the
    // full list to the `message_module` template that builds the main module.
    let mut context = Context::new();
    context.add("defs", &message_definitions);
    match TEMPLATES.render("message_module.rs", &context) {
        Ok(s) => {
            // Write the file to "mod.rs"
            let dest_path = Path::new(&out_dir).join("mod.rs");
            let mut message_module_rs = File::create(&dest_path).unwrap();
            message_module_rs.write(s.as_bytes()).unwrap();
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            for e in e.iter().skip(1) {
                eprintln!("Reason: {}", e);
            }
        }
    };

    // Loop through the message definitions and write one Rust file for each:
    for defn in message_definitions {
        match TEMPLATES.render("static_sized_message.rs", &defn) {
            Ok(s) => {
                // The file name will end up being lowercase of the ID code for that message:
                let fourcc = &defn["fourcc"].as_str().unwrap();
                let dest_path = Path::new(&out_dir).join(format!("{}.rs", fourcc.to_lowercase()));
                let mut message_rs = File::create(&dest_path).unwrap();
                message_rs.write(s.as_bytes()).unwrap();
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                for e in e.iter().skip(1) {
                    eprintln!("Reason: {}", e);
                }
            }
        };
    }
}


// Helper to open a TOML file and try and parse it
fn open_definition(filename: std::path::PathBuf) -> Result<toml::Value, TError> {

    // Read all of the file into a String
    let mut input = String::new();
    File::open(filename).and_then(|mut fh| {
        fh.read_to_string(&mut input)
    }).unwrap();

    // Return result of calling `parse`. Caller will unwrap the Result
    input.parse()
}
