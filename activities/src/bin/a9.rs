// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:



// * Use a function that returns a tuple
fn get_coords()-> (i32, i32){
  (5, 6)
}

fn main() {
  // * Destructure the return value into two variables
  // * Use an if..else if..else block to determine what to print
  let (x, y) = get_coords();

  if y > 5 {
    println!("y-value greater than 5");
  } else if y < 5 {
    println!("y-value less than 5");
  }else {
    println!("equal to 5");
  }
  println!("{:?}", x);
}
