fn main() {
  // construct tuple 
  let tup = (500, 6.4, 1);
  print_tuple(&tup);
  
  // deconstruct tuple     
  let (x, y, _) = tup;
  println!("The value of x is: {}", x);
  
  let tri_sides = compute_30_60_90_tri_side_len(y);
   // Tuple constant indexing
  assert_eq!(tri_sides.0, 6.4);
  assert_eq!(tri_sides.1, 12.8);
}

// Tuple as paramater 
fn print_tuple(x: &(i32, f64, i32)) {
       println!("The value of x is: {}", x.0);
       println!("The value of y is: {}", x.1);
       println!("The value of z is: {}", x.2);
}

// Tuple as return value 
fn compute_30_60_90_tri_side_len(short_side: f64) -> (f64, f64, f64) {
  (short_side, short_side * 2.0, short_side * 3.30 )
}
