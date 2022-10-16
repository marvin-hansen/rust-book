// Section 2.5.4 
fn main() {
    let mut s1 = String::from("Hello");
    println!("String value: {}", s1);
    
    let len = calculate_length(&s1);
    println!("String length: {}", len);
    
    change(&mut s1);
    println!("Changed value: {}", s1);
    
    let len = calculate_length(&s1);
    println!("String length: {}", len);
}

fn change(mut_string: &mut String) {
  // mut_string is a mutable reference 
    mut_string.push_str(", World");
} // mut_string goes out of scope

fn calculate_length(s: &String) -> usize { 
  // s is an immutable reference to a String
    s.len()
} // s goes out of scope, but its value will not be dropped 

// Prints 
// String value: Hello
// String length: 5
// Changed value: Hello, World
// String length: 12
// String value: Hello, World
