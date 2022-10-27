fn main() {
    let x = 5;
    let x = x + 1; // shadows within the scope

    { //Scope begins
        let x = x * 2; // shadows across scope
        println!("The value of x in the inner scope is: {x}");
    } // Scope ends

    println!("The value of x in the outer scope is: {x}");

    let spaces = "   "; // type str
    println!("{}", spaces);
    let spaces = spaces.len(); // type usize
    println!("{}", spaces);
}