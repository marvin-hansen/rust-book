const THREE_HOURS: u32 = 3;
const THREE_HOURS_IN_MINUTES: u32 = 60 * 3;

fn main() {
 let x = 5;
 println!("The value of x is: {x}");
 // x = 6; // Throws: cannot assign twice to immutable variable
 let x = 42; // shadows the previous x with a different value.
 let mut y = 5;
 println!("The value of y is: {y}");
 y = 6;
 println!("The value of y is: {y}");
}
