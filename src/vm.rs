use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;

use std::fmt;

use crate::eval::*;
use crate::func::{Function, populate_vm};
use crate::lex::*;
use crate::list::List;

#[derive(Debug, Clone, PartialEq)]
pub enum VMValue {
    Number(Rc<f64>),
    String(Rc<String>),
    Bool(Rc<bool>),
    List(Rc<List>),
    Function(Rc<Function>),
    Nil,

    EOF
}

macro_rules! vm_error {
    ($vm:expr, $($arg:tt)*) => {{
        $vm.error(format!($($arg)*));
    }};
}

impl VMValue {
    pub fn new_num(val: f64) -> VMValue {
        VMValue::Number(val.into())
    }

    pub fn new_str(val: &str) -> VMValue {
        VMValue::String(val.to_string().into())
    }

    pub fn as_num(&self) -> Result<Rc<f64>, ()> {
        match self {
            VMValue::Number(val) => Ok(val.clone()),
            _ => Err(())
        }
    }

    pub fn as_str(&self) -> Result<Rc<String>, ()> {
        match self {
            VMValue::String(val) => Ok(val.clone()),
            _ => Err(())
        }
    }
    pub fn as_list(&self) -> Result<&List, ()> {
        match self {
            VMValue::List(val) => Ok(val),
            _ => Err(())
        }
    }

    pub fn apply(self, other: Self, vm: &VMState) -> Self {
        match &self {
            VMValue::String(_) => {
                if matches!(other, VMValue::String(_)) { self.add(other, vm) }
                else { other }
            }
            VMValue::Function(f) => f.add_arg(other, vm),
            _ => other,
        }
    }

    pub fn add(self, other: Self, vm: &VMState) -> Self {
        match self {
            VMValue::String(val) => {
                let unpacked_other = other.as_str().unwrap_or_else(|_| vm_error!(vm, "Can only add string to another string"));
                let mut ret = (*val).to_owned();
                ret.push_str(&((*unpacked_other).to_owned()));
                VMValue::new_str(&ret)
            }
            VMValue::Number(val) => {
                let unpacked_other = other.as_num().unwrap_or_else(|_| vm_error!(vm, "Can only add num to another num"));
                VMValue::new_num(*val+*unpacked_other)
            },
            VMValue::List(val) => {
                match other {
                    VMValue::List(unpacked_other) => { // Append
                        VMValue::List(val.append(&unpacked_other))
                    }
                    _ => vm.error("Can only concatenate list to list")
                }
            },
            _ => vm.error("Can't add those")
        }
    }
    pub fn sub(self, other: Self, vm: &VMState) -> Self {
        match self {
            VMValue::Number(val) => {
                let unpacked_other = other.as_num().unwrap_or_else(|_| vm_error!(vm, "Can only subtract num from another num"));
                VMValue::new_num(*val-*unpacked_other)
            },
            _ => vm.error("Can't subtract those")
        }
    }
    pub fn div(self, other: Self, vm: &VMState) -> Self {
        match self {
            VMValue::Number(val) => {
                let unpacked_other = other.as_num().unwrap_or_else(|_| vm_error!(vm, "Can only divide num from another num"));
                VMValue::new_num(*val / *unpacked_other)
            },
            _ => vm.error("Can't divide those")
        }
    }
    pub fn mul(self, other: Self, vm: &VMState) -> Self {
        match self {
            VMValue::Number(val) => {
                let unpacked_other = other.as_num().unwrap_or_else(|_| vm_error!(vm, "Can only multiply num from another num"));
                VMValue::new_num(*val * *unpacked_other)
            },
            _ => vm.error("Can't multiply those")
        }
    }
}

impl fmt::Display for VMValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMValue::Number(val) => write!(f, "{}", val),
            VMValue::Bool(val) => write!(f, "{}", val),
            VMValue::String(val) => write!(f, "{}", val),
            VMValue::List(val) => write!(f, "{}", val),
            VMValue::Nil => write!(f, "nil"),
            VMValue::Function(_) => write!(f, "<function>"),
            VMValue::EOF => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct VMState {
    namespace: HashMap<String, VMValue>,
    src: Vec<String>,

    pub current_token: Token,
    pub current_contenxt: Option<Function>,
}

impl VMState {
    pub fn error(&self, msg: impl fmt::Display) -> !{
        eprintln!("Error: {}", msg);
        eprintln!("at {}:{}:", self.current_token.row, self.current_token.col);
        eprintln!("{}", self.src[self.current_token.row-1]);
        eprintln!("{:x$}^ Here", "", x = self.current_token.col-1);
        exit(1);
    }

    pub fn new() -> VMState {
        let mut ret = VMState {
            namespace: HashMap::new(),
            src: Vec::new(),
            current_token: Token {
                token: TokenType::Number(0.0),
                row:0,
                col:0
            },
            current_contenxt: Option::None,
        };
        populate_vm(&mut ret);
        ret
    }

    pub fn set(&mut self, key: &str, val: VMValue) {
        self.namespace.insert(key.to_string(), val);
    }
    
    pub fn get(&self, key: &str) -> VMValue {
        match self.namespace.get(key) {
            Some(val) => val.clone(),
            None => VMValue::Nil
        }
    }

    pub fn eval(&mut self, src: &str) -> VMValue {
        let tokens = tokenize(src, self);
        self.src = src.lines().map(|s| s.to_string()).collect();
        eval_all_tokens(tokens, self)
    }

}
