use std::{fmt, vec::Vec};
use std::rc::Rc;

use crate::vm::VMValue;
use crate::VMState;

#[derive(Debug, Clone, PartialEq)]
pub enum List {
    Pair(VMValue, Rc<List>),
    Nil
}

// Display for Lists [element, element, ...]
impl List {
    fn fmt_inner(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            List::Pair(value, next) => {
                // Print the current value followed by a comma and space if there is more
                write!(f, "{}{}", value, if let List::Pair(_, _) = &**next { ", " } else { "" })?;
                // Recursively print the rest of the list
                next.fmt_inner(f)
            }
            List::Nil => write!(f, "")
        }
    }
}
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        self.fmt_inner(f)?;
        write!(f, "]")
    }
}

impl List {
    pub fn to_vec(&self) -> Vec<VMValue> {
        let mut current = self;
        let mut ret = vec!();
        while let List::Pair(value, next) = current {
            ret.push(value.clone());
            current = &next;
        }
        ret
    }

    pub fn from_vec(vec: Vec<VMValue>) -> Rc<List> {
        let mut list = List::Nil;
        for value in vec.into_iter().rev() {
            list = List::Pair(value, Rc::new(list));
        }
        list.into()
    }

    pub fn append(&self, other: &List) -> Rc<List> {
        let mut vec = self.to_vec();
        vec.append(&mut other.to_vec());
        List::from_vec(vec)
    }

    pub fn index(&self, i: i32, vm: &VMState) -> &VMValue {
        let mut current = self;
        let mut index = i;

        while let List::Pair(value, next) = current {
            if index == 0 {
                return value;
            }
            current = &next;
            index -= 1;
        }

        vm.error("Index out of bounds")
    }
}
