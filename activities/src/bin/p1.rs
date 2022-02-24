use std::{io::{stdin, self}, collections::HashMap};

// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//

// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.


#[derive(Debug)]
struct Bills {
  name: String,
  amount: String
}

impl Bills {
  fn new() -> Result<Bills, String>{

    println!("Intro name");
    let stdin_name = get_input();

    println!("Intro amount");
    let stdin_amount = get_input();

    if stdin_name.is_ok() & stdin_amount.is_ok() {
      Ok(Bills {name: stdin_name.unwrap(), amount: stdin_amount.unwrap()})
    }else {
        Err("Error new bill".to_string())
    }
  }

  fn print_all_bills(bills: &mut HashMap<String, String>) {
    for (key, val) in bills.iter() {
      println!("name: {} amount: {}", key, val);
  }
  }

  fn remove_bill(bills: &mut HashMap<String, String>) {
    println!("Intro name");
    let stdin_name = get_input();

    if stdin_name.is_err() {
        return;
    }
  bills.remove(&stdin_name.unwrap());
  }

  fn edit_bill(bills: &mut HashMap<String, String>) {
    println!("Intro name");
    let stdin_name = get_input();

    println!("Intro new amount");
    let stdin_amount = get_input();

    for (key, val) in bills.iter_mut() {
      if key == stdin_name.as_ref().unwrap() {
        *val = stdin_amount.as_ref().unwrap().to_string()
      }
    }
  }
}


fn get_input() -> io::Result<String> {
  let mut buffer = String::new();
  stdin().read_line(&mut buffer)?;
  Ok(buffer.trim().to_owned())
}

fn option(option: String, bills: &mut HashMap<String, String>) {
  match option.to_lowercase().trim() {
    "1" => {
      let bill = Bills::new();
      match bill {
        Ok(b) => {
          bills.insert(b.name, b.amount);
        },
        Err(error) => println!("Error create: {}", error)
      }
    },
    "2" => Bills::print_all_bills(bills),
    "3" => Bills::remove_bill(bills),
    "4" => Bills::edit_bill(bills),
    _ => println!("Option not found\n")
  }
}

fn main() {
  let open = true;
  let mut bills: HashMap<String, String> = HashMap::new();

  while open {
    println!("Select an option!");
    println!("0: To leave");
    println!("1: For create new bill");
    println!("2: For print all bills");
    println!("3: For delete an invoice");
    println!("4: For edit an invoice");

    match get_input() {
      Ok(opt) => {
        match opt.as_str() {
          "0" => return,
          _ => option(opt, &mut bills)
        }
      },
      Err(error) => println!("Error: {}", error)
    }

    println!("\n")
    
  }
}
