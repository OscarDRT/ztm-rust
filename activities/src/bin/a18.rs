// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
struct Custumer {
  age: i32
}
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
fn try_purchase(custumer: &Custumer) -> Result<(), String> {
  if custumer.age >= 21 {
    Ok(())
  } else {
      Err("Under 21".to_owned())
  }
}

fn main() {
  let oscar = Custumer {age: 28};
  let purchased = try_purchase(&oscar);
  println!("{:?}", purchased)
}
