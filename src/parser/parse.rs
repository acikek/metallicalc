use std::collections::LinkedList;

use super::super::common::{types::CalcResult, log::dbg, enums::{Token, Operator}};

use Token::*;
use Operator::*;

fn precedence(token: &Token) -> u8 {
    match token {
        Op(Add) | Op(Sub) => 1,
        Op(Div) | Op(Mul) | Op(Mod) => 2,
        Op(Exp) | Op(Sci) => 3,
        _ => 0
    }
}

fn left_associative(op: &Operator) -> bool {
    match op {
        Div | Mul | Add | Sub | Mod => true,
        Exp | Sci => false
    }
}

pub fn parse(result: CalcResult<Vec<Token>>, debug: bool) -> CalcResult<LinkedList<Token>> {
    if result.is_err() { return Err(result.unwrap_err()) }

    let tokens: Vec<Token> = result.unwrap();

    // The Shunting-Yard Algorithm
    // Original implementation
    
    let mut queue = LinkedList::<Token>::new();
    let mut stack: Vec<Token> = vec![];

    for t in tokens.iter() {
        // println!("Token={:?}, Queue={:?}, Stack={:?}", t, queue, stack);

        match t {
            Num(_) => queue.push_back(*t),
            Op(o) => {
                loop {
                    // println!("Operator={:?}", o);

                    if stack.is_empty() { break; }

                    let stack_top = stack.last().unwrap();
                    let op_prec = precedence(t);
                    let top_prec = precedence(stack_top);

                    // println!("Stack is not empty. TopPrec={}, CurrentPrec={}, Queue={:?}, Stack={:?}", top_prec, op_prec, queue, stack);
                    
                    if stack_top != &LParen && (top_prec > op_prec || (op_prec == top_prec && left_associative(o))) {
                        queue.push_back(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                stack.push(*t);
            },
            LParen => stack.push(LParen),
            RParen => {
                loop {
                    if stack.is_empty() { 
                        return Err((String::from("Unmatched RParen"), None));
                    } else {
                        let stack_top = stack.pop().unwrap();

                        // println!("StackTop={:?}, Stack={:?}", stack_top, stack);

                        if stack_top == LParen { break; }
                        queue.push_back(stack_top);
                    }
                }
            }
        }
    }

    for t in stack.iter().rev() {
        if t == &LParen {
            return Err((String::from("Unmatched LParen"), None));
        } else {
            // println!("Pushing {:?} to queue", t);
            queue.push_back(*t);
        }
    }

    if debug {
        dbg(format!("Parser Queue: {:?}", queue));
    }

    return Ok(queue);
}