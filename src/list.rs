use std::{fmt, vec::Vec};
use std::ops::{Index};
use std::rc::Rc;

use crate::vm::VMValue;

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

impl Index<i32> for List {
    type Output = VMValue;
    fn index(&self, i: i32) -> &VMValue {
        let mut current = self;
        let mut index = i;

        while let List::Pair(value, next) = current {
            if index == 0 {
                return value;
            }
            current = &next;
            index -= 1;
        }

        panic!("Index out of bounds")
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
}
