use std::env;

mod vm;
mod list;

use crate::vm::*;
use crate::list::List;

fn main() {
    let mut vm = VMState::new();
    let _args: Vec<String> = env::args().collect();

    vm.set("test", VMValue::new_num(12.0));
    vm.set("test2", VMValue::new_str("Hello, World!"));

    println!("test2 is {}", vm.get("test2"));
    println!("test3 is {}", vm.get("test3"));

    let list = List::from_vec(vec!(VMValue::new_num(12.4), VMValue::new_str("GHuten morgne"), VMValue::new_num(13.0)));
    vm.set("alist", VMValue::List(list));

    let target_rc = vm.get("alist").as_list().unwrap()[1].clone();
    vm.set("test", target_rc + VMValue::new_str(" ahahahahya"));

    vm.set("alist", vm.get("alist") + VMValue::List(List::from_vec(vec!(VMValue::new_num(69.0),VMValue::new_num(420.0)))));

    println!("{}", vm.get("test"));
    println!("{}", vm.get("alist"));

    println!("{:?}", vm.get("alist").as_list().unwrap().to_vec());
}
