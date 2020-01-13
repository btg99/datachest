use colored::*;
use dc_minecraft::execute::{Datapack, Game};
use dc_minecraft::Function;
use dc_minecraft::{execute, parse, Command, FunctionIdentifier};
use regex::Regex;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::{env, io};
use zip::read::ZipArchive;

#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = env::args().collect();
    let datapack_path = match args.len() {
        1 => {
            print!("Enter datapack path: ");
            io::stdout().flush().unwrap();
            read_line()
        }
        _ => args[1].clone(),
    };
    let datapack_path = datapack_path.trim();

    match get_datapack(datapack_path) {
        Ok(mut zip_archive) => {
            let functions = get_function_files(&mut zip_archive);
            let functions = parse_functions(functions);
            let datapack = Datapack {
                name: "test".to_string(),
                functions,
            };
            let mut logger = Logger {};
            let datapack = Some(datapack);
            let mut game = Game::from(&mut logger, &datapack);
            game.execute(&Command::Function(FunctionIdentifier {
                namespace: Some("fibonacci".to_string()),
                name: "main".to_string(),
            }))
        }
        Err(FileError::File(e)) => println!("Failed to open file {}. {}", datapack_path, e),
        Err(FileError::Zip) => println!("Failed to read zip format"),
    }
}

fn read_line() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error reading text from stdin");
    input
}

fn get_datapack(datapack_path: &str) -> Result<ZipArchive<File>, FileError> {
    match File::open(datapack_path) {
        Ok(file) => match ZipArchive::new(file) {
            Ok(zip) => Ok(zip),
            Err(_) => Err(FileError::Zip),
        },
        Err(e) => Err(FileError::File(e)),
    }
}

enum FileError {
    File(io::Error),
    Zip,
}

struct McFunction {
    namespace: String,
    name: String,
    content: String,
}

struct Logger {}

impl execute::Log for Logger {
    fn log(&mut self, level: execute::Level, message: &str) {
        match level {
            execute::Level::Info => println!("{}", message.white()),
            execute::Level::Fail => println!("{}", message.red()),
        }
    }
}

fn get_function_files(archive: &mut ZipArchive<File>) -> Vec<McFunction> {
    let mut functions = Vec::new();
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        match parse_name(entry.name()) {
            Some((namespace, name)) => {
                let mut content = String::new();
                entry.read_to_string(&mut content).unwrap();
                functions.push(McFunction {
                    namespace,
                    name,
                    content,
                })
            }
            None => {}
        }
    }
    functions
}

lazy_static! {
    static ref PATH_PATTERN: Regex =
        Regex::new(r"^data/(\w+)/functions/(\w+).mcfunction$").unwrap();
}

fn parse_name(path: &str) -> Option<(String, String)> {
    match PATH_PATTERN.captures(path) {
        Some(captures) => match captures.get(1) {
            Some(namespace) => match captures.get(2) {
                Some(name) => Some((namespace.as_str().to_string(), name.as_str().to_string())),
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

fn parse_functions(mcfunctions: Vec<McFunction>) -> Vec<Function> {
    let mut functions = Vec::new();
    for func in mcfunctions {
        functions.push(Function {
            identifier: FunctionIdentifier {
                namespace: Some(func.namespace),
                name: func.name,
            },
            commands: {
                let mut commands = Vec::new();
                for text in func
                    .content
                    .split("\n")
                    .filter(|l| l.trim() != "" && !l.starts_with("#"))
                {
                    commands.push(match parse::parse_line(text.trim()) {
                        Ok(cmd) => cmd,
                        Err(_) => {
                            eprintln!("{}", text.to_string());
                            panic!();
                        }
                    });
                }
                commands
            },
        })
    }
    functions
}
