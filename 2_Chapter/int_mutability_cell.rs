// Supresses compiler warnings for demo  
// Do not in active development or production code
#![allow(dead_code)]
#![allow(unused_variables)]

// Run Rust code in playground 
// https://play.rust-lang.org/

// section 2.5.6 unique vs. shared ownership 
use std::cell::Cell;

struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    year_issued: u32,
    month_issued: u8,
    day_issued: u8,
    fcc_approved: Cell<bool>,
    fcc_identifier: Cell<String>,
}

fn main() {
    // immutable instance 
    let super_phone = PhoneModel {
        company_name: "Banana Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 6.5,
        memory: 8_000_000,
        year_issued: 2022,
        month_issued: 05,
        day_issued: 14,
        fcc_approved: Cell::new(false),
        fcc_identifier: Cell::new("".to_string()),
    };
    // Mutating internal values 
    println!("Set new value");
    let fcc_approved = false;
    super_phone.fcc_approved.set(fcc_approved);

    println!("Get new value");
    assert_eq!(super_phone.fcc_approved.get(), fcc_approved);

    println!("Replace cell value");
    let fcc_id = String::from("NI8MR450-X003");
    super_phone.fcc_identifier.replace(fcc_id.clone());

    println!("Unwrap cell value");
    let res = super_phone.fcc_identifier.into_inner();
    assert_eq!(res, fcc_id);
}
