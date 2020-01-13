use std::{env, io};
use std::io::{stdin, Write};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let datapack_path = match args.len() {
        1 => {
            print!("Enter datapack path: ");
            io::stdout().flush().unwrap();
            read_line()
        },
        n => args[1].clone(),
    };

    let mut file = File::open(datapack_path);

}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error reading text from stdin");
    input
}