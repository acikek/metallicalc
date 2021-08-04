extern crate rustyline;

mod common;
mod lexer;
mod parser;
mod interpreter;

use std::env;

use rustyline::Editor;

use lexer::lex::*;
use parser::parse::*;
use interpreter::{input::*, interpret::*};
use common::log::err;

const HELP: &str = 
r#"metallicalc, a calculator written in Rust
MIT (c) 2021 Kyle P.

Enter an arithmetic expression for evaluation.
Whitespace is completely ignored outside of
numeric sequences.

Use the '--debug' switch (-d) to view the
lexing and parsing process.

Enter 'quit' or press Ctrl+C to exit.
"#;

fn main() {
    // Args
    let mut debug = false;

    for arg in env::args() {
        let t = arg.trim();
        if t == "-d" || t == "--debug" { debug = true; }
    }

    // Intro
    println!("{}", HELP);

    let mut rl = Editor::<()>::new();

    loop {
        let res = input(&mut rl);
        
        match res {
            Ok(s) => {
                if s.trim().to_lowercase() == "quit" { break; }
        
                let tokens = lex(&s, debug);
                let parsed = parse(tokens, debug);
                let result = interpret(parsed);
        
                match result {
                    Ok(_) => (),
                    Err((e, pos)) => {
                        err(e, &s, pos);
                        continue;
                    }
                }
        
                println!("{}\n", result.unwrap());
            },
            Err(_) => break
        }
    }

    println!("Exiting...");
}