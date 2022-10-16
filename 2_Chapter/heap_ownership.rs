// Section 2.5.1 Ownership
 fn main() {
    let x = String::from("Hi");
    let y = x;
    println!("Hello: {}", x);
    println!("Hello: {}", y);
}
// Error: x value borrowed here 
// after move
