// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
  let text = "Oscar DAVID riano TaPiAs";
  // * Utilize standard library functionality to
  // transform the string to lowercase and uppercase
  println!("{:?}", text.to_uppercase());
  println!("{:?}", text.to_lowercase())
}
