mod stdlib;

use std::{collections::HashMap, fmt::Debug};
use std::fmt::{self, Formatter};

use crate::{lex::Token, VMValue, VMState};

#[derive(Clone)]
enum FunctionSource<'a> {
    FnPointer(&'a dyn Fn(Vec<VMValue>, &VMState) -> VMValue),
    Src(Vec<Token>),
}

impl Debug for FunctionSource<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FunctionSource::FnPointer(_) => write!(f, "FunctionSource::FnPointer(<fn>)"),
            FunctionSource::Src(tokens) => f.debug_tuple("FunctionSource::Src").field(tokens).finish(),
        }
    }
}

impl PartialEq for FunctionSource<'_> {
    fn eq(&self, _: &FunctionSource) -> bool {
        false
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    min_args: usize,
    
    args: Vec<String>,
    args_coll: Vec<VMValue>,
    pub arg_namespace: HashMap<String, VMValue>,

    src: FunctionSource<'static>,
}

impl Function {
    pub fn new_static(f: &'static dyn Fn(Vec<VMValue>, &VMState) -> VMValue, num: usize) -> Function {
        Function {
            min_args: num,
            
            args: Vec::new(),
            args_coll: Vec::new(),
            arg_namespace: HashMap::new(),

            src: FunctionSource::FnPointer(f),
        }
    }

    pub fn new_dynamic(tokens: Vec<Token>, num: usize) -> Function {
        Function {
            min_args: num,
            args: Vec::new(),
            args_coll: Vec::new(),
            arg_namespace: HashMap::new(),

            src: FunctionSource::Src(tokens),
        }
    }

    pub fn apply(self, vm: &VMState) -> VMValue {
        match self.src {
            FunctionSource::Src(_) => todo!(),
            FunctionSource::FnPointer(f) => f(self.args_coll, vm)
        }
    }

    pub fn add_arg(&self, other: VMValue, vm: &VMState) -> VMValue {
        let mut ret = self.clone();
        ret.args_coll.push(other);
        if ret.args_coll.len() >= ret.min_args { ret.apply(vm) }
        else { VMValue::Function(ret.into()) }
    }
}


pub fn populate_vm(vm: &mut VMState) {
    vm.set("print", VMValue::Function(Function::new_static(&stdlib::print, 1).into()));
}
