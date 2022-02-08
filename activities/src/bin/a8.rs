// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:

// * Use an enum to create different flavors of drinks
enum Flavors {
  Coffe,
  Chocolate,
}

// * Use a struct to store drink flavor and fluid ounce informatio
struct Drink {
  flavor: Flavors,
  fluid_oz: f64,
}

// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
fn print_drink(drink: Drink) {
  match drink.flavor {
    Flavors::Coffe => println!("Coffe"),
    Flavors::Chocolate => println!("Chocolate")
  }
  println!("oz: {:?}", drink.fluid_oz)
}

fn main() {
  let my_drink1 = Drink {
    flavor: Flavors::Chocolate,
    fluid_oz: 10.0,
  };

  let my_drink2 = Drink {
    flavor: Flavors::Coffe,
    fluid_oz: 1.0,
  };

  print_drink(my_drink1);
  print_drink(my_drink2)

}
