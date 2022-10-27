fn main() {
    let a = 1;
    let b = 3;
    let c = sum(a, b);
    // Try this instead of the declaration above and run again.
    // let c = 4;

    let mut list = Vec::new();
    list.push(a);
    list.push(b);
    list.push(c);

    print_type_of(&a);
    print_type_of(&b);
    print_type_of(&c);
    print_type_of(&list);
}

fn sum(x: u128, y: u128) -> u128 {
    x + y
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}