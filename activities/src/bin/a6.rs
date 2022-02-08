// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:


fn main() {
  // * Use a mutable integer variable
  let mut n = 5;
  
  // * Use a while statement
  // * Print the variable within the while loop
  // * Do not use break to exit the loop
  while n >= 1 {
    println!("{:?}", n);
    n = n - 1;
  }
  println!("DONE")
}
