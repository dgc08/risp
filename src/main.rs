use std::env;

mod vm;
use crate::vm::*;

fn main() {
    let mut vm = VMState::new();
    let _args: Vec<String> = env::args().collect();

    vm.set("test", VMValue::new_int(12.0));
    vm.set("test2", VMValue::new_str("Hello, World!".to_string()));

    println!("test2 is {}", vm.get("test2"));
    println!("test3 is {}", vm.get("test3"));

    dbg!(vm);
}
