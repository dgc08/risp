use std::iter;
use std::iter::from_fn;

use crate::VMState;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Dash,
    Star,
    Slash,
    Assign
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Literal(String),
    LeftParen,
    RightParen,

    Operator(Operator)
}

fn is_identifier_char(ch: &char) -> bool{
    match ch {
        'A' ..='Z' | 'a' ..= 'z' | '0' ..= '9' | '_' => true,
        _ => false
    }
}

pub fn tokenize(inp: &str, _vm: &VMState) ->  Vec<Token> {
    let mut ret = Vec::new();
    let mut iter = inp.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '(' => ret.push(Token::LeftParen),
            ')' => ret.push(Token::RightParen),
            '+' => ret.push(Token::Operator(Operator::Plus)),
            '-' => ret.push(Token::Operator(Operator::Dash)),
            '*' => ret.push(Token::Operator(Operator::Star)),
            '/' => ret.push(Token::Operator(Operator::Slash)),
            '=' => ret.push(Token::Operator(Operator::Assign)),
            '0'..='9' => {
                let mut number = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>();

                if iter.by_ref().next_if(|s| *s == '.').is_some() {
                    number.push('.');

                    number.extend(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())));
                }

                let n: f64 = number.parse().unwrap();
                ret.push(Token::Number(n));
            },
            'A' ..='Z' | 'a' ..= 'z' | '_' => {
                let text = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(is_identifier_char)))
                    .collect::<String>();
                ret.push(Token::Literal(text))
            }
            _ => eprintln!("Unrecognized char {}", ch)
        };
    }
    ret
}
