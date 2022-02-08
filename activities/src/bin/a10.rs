// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:

// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
fn print_result(n: bool) {
  match n {
    true => println!("its big"),
    false => println!("its small"),
  };
}


fn main() {
  // * Use a boolean variable set to the result of
  //   an if..else expression to store whether the value
  //   is > 100 or <= 100
  let number = 101;
  let is_big = number > 100;
  print_result(is_big)
}
