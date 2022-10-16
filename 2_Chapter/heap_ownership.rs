// Section 2.5.1 Ownership
fn main() {
    let x = String::from("Hi");
    let y = &x;
    let z = x.clone();
    println!("Value: {}", x);
    println!("Reference: {}", y);
    println!("Cloned value: {}", z);
}
// prints 
// Value: Hi
// Reference: Hi
// Cloned value: Hi
