use std::{
    fs,
    io::{self, Write},
    process,
};

use crate::lexer::Lexer;

pub struct Rox {
    pub had_error: bool,
}

impl Rox {
    pub fn run_file(&self, path: &str) {
        let source = fs::read_to_string(path).expect("Failed to read file");

        Rox::run(&source);

        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt() {
        let mut line = String::new();

        loop {
            print!("> ");

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");

            if line.trim().is_empty() {
                break;
            }

            Rox::run(&line);

            line.clear();
        }
    }

    pub fn run(src: &str) {
        for token in Lexer::new(src) {
            match token {
                Ok(token) => println!("{token}"),
                Err(e) => println!("Lexing error: {}", e),
            }
        }
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: i32, where_: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, where_, message);

        self.had_error = true;
    }
}
