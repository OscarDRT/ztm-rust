// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:



fn main() {
  // * Use a vector to store 4 numbers
  let vector = vec![10, 20, 30, 40];
  
  // * Iterate through the vector using a for..in loop
  // * Determine whether to print the number or print "thirty" inside the loop
  for elem in &vector {
    match elem {
        30 => println!("thirty"),
        _ => println!("{}", elem)
    }
  }
  // * Use the .len() function to print the number of elements in a vector
  println!("Number of elements: {}", vector.len())
}
