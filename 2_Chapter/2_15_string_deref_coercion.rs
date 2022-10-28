fn main() {
    let s: &str = "Hello &str";
    let mut mutable_string: String = String::from("Hello String");

    coerce_success(&mutable_string);
    //coerce_fail(s); // Uncomment to trigger compile error 
}

fn coerce_success(data: &str) { // compiles 
    println!("{}", data);
}

fn coerce_fail(data: &String) { // compile error - expected &String, but found &str
    println!("{}", data);
}
