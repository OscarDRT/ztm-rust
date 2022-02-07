// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:



fn main() {
  // * Use a variable set to either true or false
  let is_true = true;
  // * Use a match expression to determine which message to display

  match is_true {
      true => println!("it's true"),
      false => println!("it's false"),
  }
}
