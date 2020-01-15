use colored::*;
use minecraft::*;
use std::io;
use std::io::Write;

fn main() {
    let mut logger = Logger {};
    let mut chat = Chat {};
    let mut game = execute::Game::new(&mut logger, &mut chat);

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match parse::parse_line(&input.trim()) {
            Ok(command) => game.execute(&command),
            _ => {}
        }
    }
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

struct Chat {}

impl execute::Chat for Chat {
    fn tell(&mut self, players: Vec<String>, message: &str) {
        println!("{}", message.white())
    }
}
