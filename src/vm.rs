use std::collections::HashMap;
use std::rc::Rc;

use std::fmt;

use crate::list::List;

#[derive(Debug, Clone, PartialEq)]
pub enum VMValue {
    Number(Rc<f64>),
    String(Rc<String>),
    Bool(Rc<bool>),
    List(Rc<List>),
    Nil
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
    pub fn add(self, other: Self, vm: &VMState) -> Self {
        match self {
            VMValue::String(val) => {
                let unpacked_other = other.as_str().expect("Can only add string to another string");
                let mut ret = (*val).to_owned();
                ret.push_str(&((*unpacked_other).to_owned()));
                VMValue::new_str(&ret)
            }
            VMValue::Number(val) => {
                let unpacked_other = other.as_num().expect("Can only add num to another num");
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
                let unpacked_other = other.as_num().expect("Can only subtract num from another num");
                VMValue::new_num(*val-*unpacked_other)
            },
            _ => vm.error("Can't subtract those")
        }
    }
}

impl fmt::Display for VMValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMValue::Number(val) => write!(f, "{}", val),
            VMValue::Bool(val) => write!(f, "{}", val),
            VMValue::String(val) => write!(f, "\"{}\"", val),
            VMValue::List(val) => write!(f, "{}", val),
            VMValue::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub struct VMState {
    namespace: HashMap<String, VMValue>,
}

impl VMState {
    pub fn error(&self, msg: &str) -> !{
        panic!("Error: {}", msg);
    }

    pub fn new() -> VMState {
        VMState {
            namespace: HashMap::new()
        }
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
}
