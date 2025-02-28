use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum VMValue {
    Integer(Rc<f64>),
    String(Rc<String>),
    Nil
}

impl VMValue {
    pub fn new_int(val: f64) -> VMValue {
        VMValue::Integer(val.into())
    }

    pub fn new_str(val: String) -> VMValue {
        VMValue::String(val.into())
    }
}

impl std::fmt::Display for VMValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMValue::Integer(val) => write!(f, "{}", val),
            VMValue::String(val) => write!(f, "{}", val),
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
