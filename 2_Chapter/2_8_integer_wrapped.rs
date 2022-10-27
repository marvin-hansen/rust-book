fn main() {
    let x: u8 = 200;
    let y: u8 = 100;
    let res = x.wrapping_add(y);

    assert_eq!(res, 44);
    println!("Value: {}", res);
}
