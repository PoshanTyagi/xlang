mod helper;
mod lexer;
mod node;
mod parser;
mod token;

use std::io::Write;

fn main() {
    run_prompt();
}

fn run_prompt() {
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Something went Wrong...");
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Something went Wrong...");
        if line.trim() == "exit()" {
            break;
        }
        run(line);
    }
}

fn run(source: String) {
    // Lexer
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.get_tokens();

    if tokens.is_err() {
        println!("{}", tokens.unwrap_err());
        return;
    }

    // println!("{:?}", tokens.unwrap());

    // Parser
    let parser = parser::Parser::new(&tokens.unwrap());

    if parser.is_err() {
        println!("{}", parser.unwrap_err());
        return;
    }

    let node = parser.unwrap().parse();

    if node.is_err() {
        println!("{}", node.unwrap_err());
        return;
    }

    println!("{:?}", node.unwrap());
}

// fn error(line: i64, message: String) {
//     report(line, String::from(""), message);
// }
//
// fn report(line:i64, _where: String, message: String) {
//     println!("[line {}] Error {}: {}", line, _where, message);
// }
