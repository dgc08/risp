use std::iter::Peekable;

use crate::lex::*;

use crate::vm::*;

macro_rules! next_expr{
    ($iter:expr, $vm:expr) => {
        if $iter.peek().is_some() {
            eval_tokens($iter, $vm)
        }
        else {
            $vm.error("Unexpected EOF, expected Expression")
        }
    };
}

pub fn eval_all_tokens (tokens: Vec<Token>, vm: &VMState) -> VMValue {
    let mut iter = tokens.iter().peekable();

    let mut ret = VMValue::Nil;

    while iter.peek().is_some() {
        ret = eval_tokens(&mut iter, vm);
    }
    
    ret
}

pub fn exec_all_tokens (tokens: Vec<Token>, vm: &VMState) {
    let mut iter = tokens.iter().peekable();

    while iter.peek().is_some() {
        println!("{}", eval_tokens(&mut iter, vm));
    }

}

fn eval_tokens<'a> (iter: &mut Peekable<impl Iterator<Item = &'a Token>>, vm: &VMState) -> VMValue {
    if !iter.peek().is_some() {
        return VMValue::Nil
    }
    
    let token = iter.next().unwrap();
    match token {
        Token::Number(n) => {
            VMValue::new_num(*n)
        }
        Token::Literal(s) => {
            match s.as_str() { // Check keywords for values
                "nil" => VMValue::Nil,
                "true" => VMValue::Bool(true.into()),
                "false" => VMValue::Bool(false.into()),
                _ => vm.get(s)
            }
        }
        Token::LeftParen => {
            let mut sub_tokens = Vec::new();
            let mut counter = 1;

            while let Some(next_token) = iter.next() {
                if matches!(next_token, Token::RightParen) {
                    counter -= 1;
                }
                else if matches!(next_token, Token::LeftParen){
                    counter += 1;
                }

                if counter == 0 {
                    break
                }
                sub_tokens.push(next_token);
            }

            if sub_tokens.is_empty() {
                return VMValue::Nil
            }

            let mut it = sub_tokens.into_iter().peekable();
            eval_tokens(&mut it, vm)
        }
        Token::Operator(op) => {
            match op {
                Operator::Plus => {
                    let mut ret = next_expr!(iter, vm);
                    while iter.peek().is_some() {
                        ret = ret.add(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Dash => {
                    let mut ret = next_expr!(iter, vm);
                    while iter.peek().is_some() {
                        ret = ret.sub(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Slash => {
                    let mut ret = next_expr!(iter, vm);
                    while iter.peek().is_some() {
                        ret = ret.div(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Star => {
                    let mut ret = next_expr!(iter, vm);
                    while iter.peek().is_some() {
                        ret = ret.mul(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Assign => todo!()
            }
        }
        _ => {
            println!("Unhandled token: {:?}", token);
            VMValue::Nil
        }
    }
}
