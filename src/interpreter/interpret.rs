use std::collections::LinkedList;

use super::super::common::{types::CalcResult, enums::{Token, Operator}};

use Token::*;
use Operator::*;

pub fn interpret(result: CalcResult<LinkedList<Token>>) -> CalcResult<f64> {
    if result.is_err() { return Err(result.unwrap_err()) }

    let mut queue: LinkedList<Token> = result.unwrap();
    let mut stack: Vec<f64> = vec![];

    if queue.len() == 1 {
        if let Num(n) = queue.front().unwrap() {
            return Ok(*n);
        }
    } else if queue.is_empty() {
        return Err((String::from("Empty expression"), None));
    }

    while !queue.is_empty() {
        let token = queue.pop_front().unwrap();

        if let Num(n) = token {
            stack.push(n);
        } else {
            // println!("INIT EVAL, Stack={:?}", stack);

            let o2 = stack.pop();
            let o1 = stack.pop();

            if o1.is_none() || o2.is_none() {
                return Err((String::from("Unfinished expression"), None));
            }

            let n1 = o1.unwrap();
            let n2 = o2.unwrap();

            let result = match token {
                Op(Add) => n1 + n2,
                Op(Sub) => n1 - n2,
                Op(Mul) => n1 * n2,
                Op(Div) => n1 / n2,
                Op(Exp) => n1.powf(n2),
                Op(Sci) => n1 * (10_f64.powf(n2)),
                Op(Mod) => n1 % n2,
                _ => 0.0 
            };

            // println!("END EVAL, Result={}, Stack={:?}", result, stack);

            stack.push(result);
        }
    }

    // println!("END, Stack={:?}", stack);

    if stack.len() > 1 {
        return Err((String::from("Imbalanced expression"), None));
    }

    return Ok(*stack.first().unwrap());
}