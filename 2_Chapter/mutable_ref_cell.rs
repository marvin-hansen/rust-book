// Run Rust code in playground 
// https://play.rust-lang.org/

// Section 2.5.6 unique vs. shared ownership 
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new("foo".to_string());
    
    {
        let imut_ref = cell.borrow(); // ok!
        print_string(&imut_ref);
    }   // imut_ref goes out of scope 
    
    {
        let mut mut_ref = cell.borrow_mut();
        *mut_ref = "bar".to_string();
        print_string(&mut_ref);
    }   // mut_ref goes out of scope
    
    {
        let mut mut_ref = cell.borrow_mut(); // ok!
        *mut_ref = "bobo".to_string();
         print_string(&mut_ref);
    }   // mut_ref goes out of scope

  let last_imut_ref = cell.borrow(); // ok!
  println!("{}", last_imut_ref);    

} // cell goes out of scope 

fn print_string(s: &String){
        println!("{}", s);
}
