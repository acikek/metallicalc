extern crate colored;

use colored::*;

pub fn switch(s: &str, v: &mut bool) {
    *v = !(*v);
    println!("Switch '{}' set to {}\n", s, v);
}

pub fn dbg(s: String) {
    println!("{} {}", "DBG:".green(), s);
}

pub fn err(e: String, s: &String, pos: Option<usize>, t: bool) {
    let prefix = "ERR!";
    
    let trace: String = if t { 
        let spacing = format!("{}{} ", " ".repeat(prefix.len() - 1), "|".red());

        match pos {
            Some(p) => format!("\n{}\n{}{}\n{}{}{}", spacing, spacing, s, spacing, " ".repeat(p - 1), "^"),
            None => String::new()
        }
    } else { 
        String::new() 
    };

    println!("{} {}{}\n", "ERR!".red(), e, trace);
}