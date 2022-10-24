// Section 2.5.7 Shared ownership

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));
    println!("Original value = {:?}", value);

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    print_ref_count("ref count after creating a", &a);

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    print_ref_count("ref count after creating b", &a);

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    print_ref_count("ref count after creating c", &a);

    *value.borrow_mut() += 10;
    println!("Updated value = {:?}", value);

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn print_ref_count(msg: &str, a: &Rc<List>){
    println!("{} = {}", msg, Rc::strong_count(a));
}
//  Prints 
//  Original value = RefCell { value: 5 }
// ref count after creating a = 1
// ref count after creating b = 2
// ref count after creating c = 3
// Updated value = RefCell { value: 15 }
// a after = Cons(RefCell { value: 15 }, Nil)
// b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
// c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
