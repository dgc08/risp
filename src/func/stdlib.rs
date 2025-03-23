use crate::{VMValue, VMState};

pub fn print(args: Vec<VMValue>, _vm: &VMState) -> VMValue {
    let mut ret = VMValue::Nil;
    for i in args {
        print!("{} ", i);
        ret = i;
    }
    println!("");
    ret
}
