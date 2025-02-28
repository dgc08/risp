use std::env;

mod vm;
mod list;

use crate::vm::*;
use crate::list::List;

fn main() {
    let mut vm = VMState::new();
    let _args: Vec<String> = env::args().collect();

    // vm.set("test", VMValue::new_num(12.0));
    // vm.set("test2", VMValue::new_str("Hello, World!".to_string()));

    // println!("test2 is {}", vm.get("test2"));
    // println!("test3 is {}", vm.get("test3"));

    // let parlist = List::Pair(VMValue::new_str("hi".to_string()), List::Nil.into()).into();
    // vm.set("alist", VMValue::List(List::Pair(VMValue::new_num(12.0), parlist).into()));

    let list = List::from_vec(vec!(VMValue::new_num(12.0), VMValue::new_num(13.0), VMValue::new_str("GHuten morgne")));
    vm.set("alist", VMValue::List(list));

    println!("{}", vm.get("alist"));

    //vm.set("alist", VMValue::new_str("Hello, Wolrd!?".to_string()));
    println!("{:?}", vm.get("alist").as_list().unwrap().to_vec());

    //dbg!(vm);
}
