use std::iter;
use std::iter::from_fn;

use crate::VMState;

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Dash,
    Star,
    Slash,
    Assign
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Number(f64),
    Literal(String),
    String(String),
    LeftParen,
    RightParen,

    Operator(Operator)
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub row: usize,
    pub col: usize
}

fn is_identifier_char(ch: &char) -> bool{
    match ch {
        'A' ..='Z' | 'a' ..= 'z' | '0' ..= '9' | '_' => true,
        _ => false
    }
}

fn parse_string(iter: &mut impl Iterator<Item = char>, vm: &VMState) -> (String, usize) {
    let mut ret: Vec<char> = Vec::new();
    let mut d_col = 0;

    while let Some(c) = iter.next() {
        d_col += 1;
        match c {
            '"' => return (ret.into_iter().collect(), d_col),
            '\n' => vm.error("Undetermined String"),
            '\\' => ret.push({
                d_col += 1;
                let c = iter.next().unwrap_or_else(|| vm.error("Undetermined String"));
                match c {
                    '0' => '\0',
                    't' => '\t',
                    'n' => '\n',
                    'r' => '\r',
                    '\\' => '\\',
                    _ => vm_error!(vm, "Unknown escape code '{}'", c)
                }
            }),
            _ => ret.push(c)
        }
    }

    vm.error("Undeterminated string")
}

pub fn tokenize(inp: &str, vm: &mut VMState) ->  Vec<Token> {
    let mut ret = Vec::new();

    let mut col = 0;
    let mut row = 1; // start on row one as well

    let mut iter = inp.chars().peekable();

    while let Some(ch) = iter.next() {
        col += 1;
        let token =
        match ch {
            '\n' => {row+=1; col = 0; continue;},
            ch if ch.is_whitespace() => continue, // else whitespace

            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '+' => TokenType::Operator(Operator::Plus),
            '-' => TokenType::Operator(Operator::Dash),
            '*' => TokenType::Operator(Operator::Star),
            '/' => TokenType::Operator(Operator::Slash),
            '=' => TokenType::Operator(Operator::Assign),
            ';' => {
                iter.by_ref().take_while(|&ch| ch != '\n').for_each(|_| {});
                col = 0; row += 1;
                continue;
            },
            '0'..='9' => {
                let mut number = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>();

                if iter.by_ref().next_if(|s| *s == '.').is_some() {
                    number.push('.');

                    number.extend(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())));
                }

                let n: f64 = number.parse().unwrap();
                TokenType::Number(n)
            },
            'A' ..='Z' | 'a' ..= 'z' | '_' => {
                let text = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(is_identifier_char)))
                    .collect::<String>();
                TokenType::Literal(text)
            }
            '"' => {
                let (s, d_col) = parse_string(&mut iter, vm);
                col += d_col;
                TokenType::String(s)
            }
            _ => vm_error!(vm, "Unexpected char {}", ch)
        };
        let t = Token {
            token,
            row,
            col
        };
        vm.current_token = t.clone();
        ret.push(t)
    }
    ret
}
