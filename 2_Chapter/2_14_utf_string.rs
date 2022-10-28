fn main() {
    let hello = String::from("السلام عليكم");
    print_something(&hello);
    let hello = String::from("Dobrý den");
    print_something(&hello);
    let hello = String::from("Hello");
    print_something(&hello);
    let hello = String::from("שָׁלוֹם");
    print_something(&hello);
    let hello = String::from("नमस्ते");
    print_something(&hello);
    let hello = String::from("こんにちは");
    print_something(&hello);
    let hello = String::from("안녕하세요");
    print_something(&hello);
    let hello = String::from("你好");
    print_something(&hello);
    let hello = String::from("Olá");
    print_something(&hello);
    let hello = String::from("Здравствуйте");
    print_something(&hello);
    let hello = String::from("Hola");
    print_something(&hello);
}

fn print_something(something: &str) {
    println!("{}", something);
}
