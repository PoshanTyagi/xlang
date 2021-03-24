mod Helper;
mod Lexer;
mod Token;
mod TokenType;

use crate::Lexer::*;

use std::io::Write;

fn main() {
    run_prompt();
}

fn run_prompt() {
    loop {
        print!("> ");
        std::io::stdout().flush();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        if line.trim() == "exit()" {
            break;
        }
        run(line);
    }
}

fn run(source: String) {
    let mut lexer = Lexer::Lexer::new(source);
    let tokens = lexer.get_tokens();

    match tokens {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}

// fn error(line: i64, message: String) {
//     report(line, String::from(""), message);
// }
//
// fn report(line:i64, _where: String, message: String) {
//     println!("[line {}] Error {}: {}", line, _where, message);
// }
