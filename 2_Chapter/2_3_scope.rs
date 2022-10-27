fn main() {
    let outter = 5;
    println!("The value is: {outter}");

    {
        let inner = 6;
        println!("The value is: {inner}");
    } // inner scope ends
}