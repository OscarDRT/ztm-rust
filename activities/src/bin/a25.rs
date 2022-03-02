// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
trait Perimeter {
  fn calculate_perimeter(&self) -> i32;
}

struct Square {
  side: i32
}

impl Perimeter for Square {
  fn calculate_perimeter(&self) -> i32 {
    self.side * 4
  }
} 

struct Triangle {
  side_a: i32,
  side_b: i32,
  side_c: i32

}

impl Perimeter for Triangle {
  fn calculate_perimeter(&self) -> i32 {
    self.side_a + self.side_b + self.side_c
  }
}
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter
fn print_perimeter(shape: impl Perimeter) {
  let perimeter = shape.calculate_perimeter();
  println!("Perimeter: {:?}", perimeter)
}

fn main() {
  print_perimeter(Square {side: 10});
  print_perimeter(Triangle {side_a: 10, side_b: 10, side_c: 10})
}
