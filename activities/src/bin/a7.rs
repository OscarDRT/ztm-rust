// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:



// * Use an enum with color names as variants
enum Colors {
    BLue,
    Red,
    Black,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
fn print_color(my_comlor: Colors) {
  match my_comlor {
    Colors::BLue => println!("Blue"),
    Colors::Red => println!("Red"),
    Colors::Black => println!("Black"),
  }
}

fn main() {
  let color = Colors::BLue;
  print_color(color);
}
