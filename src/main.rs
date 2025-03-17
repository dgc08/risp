use std::env;

mod vm;
mod list;
mod lex;
mod eval;

use eval::eval_all_tokens;
use lex::tokenize;

use crate::vm::*;
use crate::list::List;

fn main () {
    let mut vm = VMState::new();
    let _args: Vec<String> = env::args().collect();

    vm.set("test", VMValue::new_num(12.0));
    vm.set("test2", VMValue::new_str("Hello, World!"));

    // println!("test2 is {}", vm.get("test2"));
    // println!("test3 is {}", vm.get("test3"));

    let list = List::from_vec(vec!(VMValue::new_num(12.4), VMValue::new_str("GHuten morgne"), VMValue::new_num(13.0)));
    vm.set("alist", VMValue::List(list));

    let binding = vm.get("alist");
    let target_r = binding.as_list().unwrap();
    let target = target_r.index(1, &vm).clone();
    vm.set("test", target.add(VMValue::new_str(" ahahahahya"), &vm));

    // println!("{}", vm.get("test"));
    // println!("{}", vm.get("alist"));

    // println!("{:?}", vm.get("alist").as_list().unwrap().to_vec());

    // println!("-------------------------------------------------------");

    //let tokens = tokenize("1.0 + 2.3 / 2 _a2mogus_ga".to_string(), &vm);
    let src = "- (+ 2 11) 1.5";
    println!("SRC: {}", src);
    let tokens = tokenize(src.to_string(), &vm);
    // println!("Tokens: {:?}", tokens);
    println!("Eval to: {:?}", eval_all_tokens(tokens, &vm));
}
