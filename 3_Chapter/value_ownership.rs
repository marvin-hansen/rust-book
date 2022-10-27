fn print_string(s: String) {
    println!("{}", s);
} // owner and s goe out of scope

fn main() {
    let s = String::from("Hello, World");
    print_string(s);  // ownership of s moved to print_string
    // another print_string(s); would result in an error
}