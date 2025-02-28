use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

use crate::list::List;

#[derive(Debug, Clone, PartialEq)]
pub enum VMValue {
    Number(Rc<f64>),
    String(Rc<String>),
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

    pub fn as_num(&self) -> Result<f64, ()> {
        match self {
            VMValue::Number(val) => Ok(**val),
            _ => Err(())
        }
    }

    pub fn as_str(&self) -> Result<String, ()> {
        match self {
            VMValue::String(val) => Ok((**val).clone()),
            _ => Err(())
        }
    }
    pub fn as_list(&self) -> Result<&List, ()> {
        match self {
            VMValue::List(val) => Ok(val),
            _ => Err(())
        }
    }
}

impl fmt::Display for VMValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMValue::Number(val) => write!(f, "{}", val),
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
