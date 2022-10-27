// This code does not compile. 
// Type BoundedU8 has been skipped for brevity. 
// See section 2.8: Type constraint in the book for a working example.  
// https://rust-academy.com/free-book/
type Element = BoundedU8<1, 200>; // Type alias with value constraint 

fn main() {
    let value = Element::new(10);
    println!("Bounded x = {}", value.unwrap().0);

    let err_val = Element::new(250);
    match err_val { 
         // graceful error handling
        Ok(x) => { println!("x = {}", x.0); }
        Err(err) => {eprintln!("Error: {}", err)}
    }
}
