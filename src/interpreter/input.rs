use std::io::{ stdin, stdout, Write };

pub fn input() -> String {
    let mut result = String::new();

    print!("> ");
    let _ = stdout().flush();

    stdin().read_line(&mut result).expect("Invalid input");

    let trimmed = String::from(
        result
            .strip_suffix("\r\n")
            .or(result.strip_suffix("\n"))
            .unwrap()
    );

    return trimmed;
}

pub fn err(e: String) {
    println!("ERR! {}\n", e);
}