use std::fmt::Debug; 

fn main() {
    let x = Box::new(5);
    println!("x = {}", x); // pass by value
    
    print_box(&x) // pass by reference 
}   // x goes out of scope and will be deallocated

fn print_box<T: Debug>(b: &Box<T>){
    println!("Box content = {:?}", b);
}
