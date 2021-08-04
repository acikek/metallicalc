extern crate colored;

use colored::*;

pub fn dbg(s: String) {
    println!("{} {}", "DEBUG:".green(), s);
}

pub fn err(e: String, s: &String, pos: Option<u16>) {
    println!("{} {}\n", "ERR!".red(), e);
}