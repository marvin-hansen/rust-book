const CONST_STRING: &'static str = "A constant string";

fn main() {
    let s = "hello_world";
    let string_value = String::from("hello");

    print_something(&string_value);
    print_something(s);
    print_something(&s[0..6]);
    print_something(CONST_STRING);
}

fn print_something(something: &str) {
    println!("{}", something);
}
