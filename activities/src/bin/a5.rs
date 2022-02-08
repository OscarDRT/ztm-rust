// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:



fn main() {
  // * Use a mutable integer variable
  let mut n = 1;

// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop
loop {
  println!("{:?}", n);
    if n == 4 {
      break;
    }
  n = n + 1;
}

}
