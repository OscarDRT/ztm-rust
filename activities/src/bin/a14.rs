// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:


// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
  age: i32,
  name: String,
  color: String,
}



// * The name and colors should be printed using a function
fn print_person(name: &str, color: &str) {
  println!("name: {}, color: {}", name, color)
}
fn main() {
  // * Create and store at least 3 people in a vector
  let people = vec![
    Person {
      age: 28,
      name: "Oscar".to_owned(),
      color: "Black".to_owned(),
    },
    Person {
      age: 8,
      name: String::from("David"),
      color: String::from("Red"),
    },
    Person {
      age: 10,
      name: String::from("Pedro"),
      color: String::from("Blue"),
    },
  ];

  for person in people {
    if person.age <= 10 {
      print_person(&person.name, &person.color);
    }
  }
}
