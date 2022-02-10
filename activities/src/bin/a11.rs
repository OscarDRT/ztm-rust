// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct Item {
  quantity: i32,
  id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(i: &Item) {
  println!("Quantity: {:?}", i.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(i: &Item) {
  println!("Id: {:?}", i.id);
}

fn main() {
  let item = Item {
    quantity: 32,
    id: 1234,
  };
  display_quantity(&item);
  display_id(&item);
}