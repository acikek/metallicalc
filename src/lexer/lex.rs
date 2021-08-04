use super::super::common::{types::CalcResult, log::dbg, enums::{Token, Operator}};

pub fn lex(s: &String, debug: bool) -> CalcResult<Vec<Token>> {
    use Token::*;
    use Operator::*;

    let trimmed = s.trim();

    let mut result: Vec<Token> = vec![];

    // The current number sequence.
    let mut current = String::new();

    // The sequence mode flags.
    let mut num_start = false;
    let mut num_end = false;
    
    for (i, c) in trimmed.chars().enumerate() {
        let position = i + 1;
        let is_end = position == trimmed.len();
        
        // println!("'{}' at {}. num_start={}, num_end={}, is_end={}, position={}, len={}", c, i, num_start, num_end, is_end, position, s.len());

        if c == '.' {
            // If there is no current number sequence,
            // or there has already been a decimal point recorded,
            // it is an invalid sequence.
            if !num_start || num_end {
                return Err((format!("Invalid token after '{}' at position {}", current, position), Some(position)));
            }
            
            current.push(c);

            // Switch the sequence mode.
            num_start = false;
            num_end = true;
            
            if !is_end { continue; }
        } else if c.is_numeric() {
            current.push(c);
            if !num_start { num_start = true; }

            // If this is the last character in the sequence, 
            // it needs to be checked and appended.
            if !is_end { continue; }
        }

        // If by this point there is just the end (.) and no
        // trailing numbers, then the sequence is incomplete.
        if num_end && !num_start {
            return Err((format!("Unfinished numeric sequence '{}' at position {}", current, position), Some(position)));
        }

        if num_start || num_end {
            let num = current.parse::<f64>().unwrap();

            result.push(Num(num));
            current.clear();
        
            num_start = false;
            num_end = false;

            // If the last character is a number sequence,
            // no further token parsing is required.
            if is_end && c.is_numeric() { break; }
        }

        // Ignore whitespace outside of numbers.
        if c == ' ' && !num_start && !num_end { continue; }

        // Check for valid tokens.
        let parsed: Result<Token, String> = match c {
            '+' => Ok(Op(Add)),
            '-' => Ok(Op(Sub)),
            '*' => Ok(Op(Mul)),
            '/' => Ok(Op(Div)),
            '^' => Ok(Op(Exp)),
            'e' => Ok(Op(Sci)),
            '%' => Ok(Op(Mod)),
            '(' => Ok(LParen),
            ')' => Ok(RParen),
            _ => Err(format!("Invalid token '{}' at position {}", c, position))
        };

        if parsed.is_err() { return Err((parsed.unwrap_err(), Some(position))) }

        result.push(parsed.unwrap());
    }

    if debug {
        dbg(format!("Lexer Output: {:?}", result));
    }

    return Ok(result);
}