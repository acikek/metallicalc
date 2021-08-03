mod common;
mod lexer;
mod parser;
mod interpreter;

use std::env;

use lexer::lex::*;
use parser::parse::*;
use interpreter::{input::*, interpret::*};

fn main() {
    // Args
    let mut debug = false;

    for arg in env::args() {
        let t = arg.trim();
        if t == "-d" || t == "--debug" { debug = true; }
    }

    // Intro
    println!(
"metallicalc, a calculator written in Rust
MIT (c) 2021 Kyle P.

Enter an arithmetic expression for evaluation.
Whitespace is completely ignored outside of
numeric sequences.

Use the '--debug' switch (-d) to view the
lexing and parsing process.

Enter 'quit' or press Ctrl+C to exit.
    ");

    loop {
        let s = input();

        if s.trim().to_lowercase() == "quit" { break; }
        
        let tokens = lex(s, debug);
        let parsed = parse(tokens, debug);
        let result = interpret(parsed);

        match result {
            Ok(_) => (),
            Err(e) => {
                err(e);
                continue;
            }
        }

        println!("{}\n", result.unwrap());
    }

    println!("Exiting...");
}