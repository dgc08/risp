use std::env;
use std::fs;

#[macro_use]
mod vm;

mod list;
mod func;
mod lex;
mod eval;

use crate::vm::*;
use crate::list::List;

fn main () {
    let mut vm = VMState::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl(&mut vm);
    }
    else {
        let contents = fs::read_to_string(&args[1]).unwrap_or_else(|e| vm_error!(vm, "Could not open file: {}", e));
        vm.eval(&contents);
    }

}

fn repl(vm: &mut VMState) { //TODO
    vm.set("test", VMValue::new_num(12.0));
    vm.set("test2", VMValue::new_str("Hello, World!"));

    println!("test2 is {}", vm.get("test2"));
    println!("test3 is {}", vm.get("test3"));

    let list = List::from_vec(vec!(VMValue::new_num(12.4), VMValue::new_str("GHuten morgne"), VMValue::new_num(13.0)));
    vm.set("alist", VMValue::List(list));

    let binding = vm.get("alist");
    let target_r = binding.as_list().unwrap();
    let target = target_r.index(1, &vm).clone();
    vm.set("test", target.add(VMValue::new_str(" ahahahahya"), &vm));

    println!("{}", vm.get("test"));
    println!("{}", vm.get("alist"));

    println!("{:?}", vm.get("alist").as_list().unwrap().to_vec());

    println!("-------------------------------------------------------");

    let src = "- (+ 2 11) 1.5";
    println!("SRC: {}", src);
    println!("Eval to: {:?}", vm.eval(src));
}
