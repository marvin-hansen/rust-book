fn main() {
    let arg = std::env::args().skip(1).next()
        .expect("should have one argument");
    println!("upp = {}", arg.to_uppercase());
}
// cargo run --quiet -- "Приветствую, мир"
// Prints: upp = ПРИВЕТСТВУЮ, МИР
