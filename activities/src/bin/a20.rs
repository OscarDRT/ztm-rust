use std::io::*;
// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
enum PowerState {
  Off,
  Sleep,
  Reboot,
  Shutdown,
  Hibernate
}

// * Use a match expression to convert the user input into the power state enum
impl PowerState {
  fn new(state: &str) -> Option<PowerState> {
    match state.to_lowercase().trim() {
      "off" => Some(PowerState::Off),
      "sleep" => Some(PowerState::Sleep),
      "reboot" => Some(PowerState::Reboot),
      "shutdown" => Some(PowerState::Shutdown),
      "hibernate" => Some(PowerState::Hibernate),
      _ => None
    }
  }
}

// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
fn print_action(state: PowerState) {
  use PowerState::*;
  match state {
    Off => println!("-> Off <-"),
    Sleep => println!("-> Sleep <-"),
    Reboot => println!("-> Reboot <-"),
    Shutdown => println!("-> Shutdown <-"),
    Hibernate => println!("-> Hibernate <-"),
  }
}

// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

fn main() {
  let mut buffer = String::new();
  let user_input = stdin().read_line(&mut buffer);
  if user_input.is_ok() {
    match PowerState::new(&buffer) {
      Some(state) => print_action(state),
      None => println!("Invalid statate")
    }
  } else {
    println!("Input Error")
  }
}
