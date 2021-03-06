extern crate rustyline;

mod common;
mod lexer;
mod parser;
mod interpreter;

use std::collections::HashMap;

use rustyline::Editor;

use lexer::lex::*;
use parser::parse::*;
use interpreter::{input::*, interpret::*};
use common::log::*;

const INTRO: &str = 
r#"metallicalc, a calculator written in Rust
MIT (c) 2021 Kyle P.

Enter an arithmetic expression for evaluation.
Whitespace is completely ignored outside of
numeric sequences.

Run 'help' for more info. Enter 'quit' or 
press Ctrl+C to exit.
"#;

const HELP: &str = 
r#"cache -   toggles expr caching
cc    -      clears expr cache
clear -    clears line history
dc    -    displays expr cache
debug - toggles debug messages
help  -  displays this message
quit  -      exits the program
trace -   toggles error traces
"#;

fn main() {
    // Switches
    let mut debug = false;
    let mut trace = true;
    let mut cache_v = true;

    // Intro
    println!("{}", INTRO);

    let mut rl = Editor::<()>::new();
    let mut cache = HashMap::<String, f64>::new();

    loop {
        let i = input(&mut rl);
        
        match i {
            Ok(s) => {
                let mut cmd = true;
                let t = s.trim();
                let stripped = strip_whitespace(t);

                // Commands
                match t.to_lowercase().as_str() {
                    "cache" => { switch("cache", &mut cache_v); }
                    "cc" => {
                        cache.clear();
                        println!("Cache cleared\n");
                    }
                    "clear" => { 
                        rl.clear_history();
                        println!("Line history cleared\n");
                    },
                    "dc" => {
                        if cache.is_empty() {
                            println!("Nothing in cache\n");
                        } else {
                            let list: Vec<String> = cache.clone().into_iter().map(|(k, v)| format!("{} => {}", k, v)).collect();
                            println!("{}\n", list.join("\n"));
                        }
                    },
                    "debug" => { switch("debug", &mut debug) },
                    "help" => { println!("{}", HELP) },
                    "quit" => break,
                    "trace" => { switch("trace", &mut trace) },
                    _ => { cmd = false; }
                };

                if cmd { continue };

                if cache_v && cache.contains_key(&stripped) {
                    let v = *cache.get(&stripped).unwrap();
                    if debug { dbg(format!("Cached string '{}' returned {}", stripped, v)); }
                    res(v); continue;
                }
        
                let tokens = lex(&s, debug);
                let parsed = parse(tokens, debug);
                let result = interpret(parsed);
        
                match result {
                    Ok(_) => (),
                    Err((e, pos)) => {
                        err(e, &s, pos, trace);
                        continue;
                    }
                }
                
                let n = result.unwrap();

                if cache_v { cache.insert(stripped, n); }
                res(n);
            },
            Err(_) => break
        }
    }

    println!("Exiting...");
}