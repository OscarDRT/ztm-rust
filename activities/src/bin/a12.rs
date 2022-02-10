// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//


struct Dimensions {
  width: f64,
  height: f64,
  depth: f64
}

impl Dimensions {
  fn print(&self) {
    println!("width: {}", self.width);
    println!("height: {}", self.height);
    println!("depth: {}", self.depth)
  } 
}


// * Use an enum for the box color
enum Color {
  Red,
  Blue
}

impl Color {
  fn print(&self) {
    match self {
      Color::Blue => println!("Blue"),
      Color::Red => println!("Red"),
      
    }
  }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
  weight: f64,
  color: Color,
  dimensions: Dimensions
}

impl ShippingBox {
  // * Implement functionality on the box struct to create a new box
  fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
    Self {
      weight,
      color,
      dimensions
    }
  }
  // * Implement functionality on the box struct to print the characteristics
  fn print(&self) {
    println!("characteristics:");
    println!("weight: {}", &self.weight);
    Color::print(&self.color); // self.color.print();
    self.dimensions.print() // Dimensions::print(&self.dimensions)

  }
}



fn main() {
  let box1 = ShippingBox::new(10.2, Color::Blue, Dimensions {
    width:10.1,height: 2.2, depth: 2.2
  });
  ShippingBox::print(&box1);

  let box2 = ShippingBox::new(10.2, Color::Red, Dimensions {
    width:10.1,height: 2.2, depth: 2.2
  });
  box2.print()
}
