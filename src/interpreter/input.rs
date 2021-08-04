extern crate colored;
extern crate rustyline;

use colored::*;
use rustyline::{Editor, error::ReadlineError};

pub fn input(rl: &mut Editor::<()>) -> Result<String, ReadlineError> {
    let result = rl.readline(format!("{}", "=> ".bright_blue()).as_str());

    return match result {
        Ok(line) => {
            let s = line.as_str();
            let trimmed = String::from(s
                .strip_suffix("\r\n")
                .or(s.strip_suffix("\n"))
                .unwrap_or(s)
            );

            rl.add_history_entry(&trimmed);
            Ok(trimmed)
        },
        Err(_) => result
    };
}