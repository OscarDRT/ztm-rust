// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:


// * Use an enum for the tickets with data associated with each variant
enum Tickets {
  Backstage(f64, String),
  Vip(f64, String),
  Standard(f64)
}


fn main() {
  // * Create one of each ticket and place into a vector
  let tickets = vec![
    Tickets::Backstage(200.0, "Oscar".to_owned()),
    Tickets::Vip(100.0, "David".to_owned()),
    Tickets::Standard(50.0),
  ];

  // * Use a match expression while iterating the vector to print the ticket info
  for ticket in tickets {
    match ticket {
      Tickets::Backstage(price, holder) => println!("Price: {:?} - Holder: {:?} - Backstage", price, holder),
      Tickets::Vip(price, holder) => println!("Price: {:?} - Holder: {:?} - Vip", price, holder),
      Tickets::Standard(price) => println!("Price: {:?} - Standard", price),
    }
  }
}
