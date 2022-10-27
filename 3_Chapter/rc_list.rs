// Supresses compiler warnings for demo  
// Do not in active development or production code
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    print_ref_count("ref count after creating a", &a);

    { // Inner scope I
        let b = Cons(3, Rc::clone(&a));
        print_ref_count("ref count after creating b", &a);
        let c = Cons(4, Rc::clone(&a));
        print_ref_count("ref count after creating c", &a);
    }
    print_ref_count("ref count after b & c go out of scope", &a);

    { // Inner scope II 
        let e = Cons(5, Rc::clone(&a));
        print_ref_count("ref count after creating e", &a);
    }
    print_ref_count("ref count after e goes out of scope", &a);
    
} // end of nain scope, ref count drops to zero, and a and will be dropped 

fn print_ref_count(msg: &str, a: &Rc<List>){
    println!("{} = {}", msg, Rc::strong_count(a));
}
