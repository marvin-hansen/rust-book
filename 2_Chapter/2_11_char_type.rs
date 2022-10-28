fn main() {
    let s = String::from("love: ❤️");
    let v: Vec<char> = s.chars().collect();
    
    assert_eq!(12, std::mem::size_of_val(&s[..]));
    assert_eq!(32, std::mem::size_of_val(&v[..]));
}
