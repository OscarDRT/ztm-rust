// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//

// * Use the println macro to display messages to the terminal

fn main() {
  let my_number = 6;
// * Use an if..else if..else block to determine which message to display
if my_number > 5 {
  println!(">5");
} else if my_number < 5 {
    println!("<5");
}else {
    println!("=5");
}
}
