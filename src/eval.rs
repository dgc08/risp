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

macro_rules! expr_available {
    ($iter:expr) => { !matches!($iter.peek().cloned().cloned().unwrap_or_default().token, TokenType::Semi) };
}

pub fn eval_all_tokens (tokens: Vec<Token>, vm: &mut VMState) -> VMValue {
    let mut iter = tokens.iter().peekable();

    let mut ret = VMValue::Nil;

    while iter.peek().is_some() {
        let expr = eval_tokens(&mut iter, vm);
        if matches!(expr, VMValue::EOF) {
            return ret
        }
        ret = ret.apply(expr, vm);
    }
    
    ret
}

fn eval_tokens<'a> (iter: &mut Peekable<impl Iterator<Item = &'a Token>>, vm: &mut VMState) -> VMValue {
    if !iter.peek().is_some() {
        return VMValue::Nil
    }
    
    let token = iter.next().unwrap();
    vm.current_token = token.clone();
    match &token.token {
        TokenType::Semi => {
            if expr_available!(iter) {
                eval_tokens(iter, vm)
            }
            else if iter.peek().is_none() { VMValue::EOF }
            else { VMValue::Nil }
        }
        TokenType::Number(n) => {
            VMValue::new_num(*n)
        }
        TokenType::String(s) => {
            VMValue::new_str(s)
        } 
        TokenType::Literal(s) => {
            match s.as_str() { // Check keywords for values
                "nil" => VMValue::Nil,
                "true" => VMValue::Bool(true.into()),
                "false" => VMValue::Bool(false.into()),
                _ => vm.get(s)
            }
        }
        TokenType::RightParen => vm.error("Unmatched '('"),
        TokenType::LeftParen => {
            let mut sub_tokens = Vec::new();
            let mut counter = 1;

            while let Some(next_token) = iter.next() {
                match next_token.token {
                    TokenType::RightParen => counter -= 1,
                    TokenType::LeftParen  => counter += 1,
                    _ => ()
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
        TokenType::Operator(op) => {
            match op {
                Operator::Plus => {
                    let mut ret = next_expr!(iter, vm);
                    while expr_available!(iter) {
                        ret = ret.add(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Dash => {
                    let mut ret = next_expr!(iter, vm);
                    while expr_available!(iter) {
                        ret = ret.sub(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Slash => {
                    let mut ret = next_expr!(iter, vm);
                    while expr_available!(iter) {
                        ret = ret.div(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Star => {
                    let mut ret = next_expr!(iter, vm);
                    while expr_available!(iter) {
                        ret = ret.mul(next_expr!(iter, vm), vm);
                    }
                    ret
                },
                Operator::Assign => {
                    let token = iter.next().unwrap_or_else(|| vm.error("Unexpected EOF, expected Identifier"));
                    let TokenType::Literal(ref ident) = token.token else { vm.error("Unexpected EOF, expected Identifier") };

                    let val = next_expr!(iter, vm);
                    vm.set(ident, val.clone());
                    val
                }
            }
        }
    }
}
