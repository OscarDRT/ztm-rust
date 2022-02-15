// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
enum Position {
  Maintenance,
  Marketing,
  Managers,
  LineSupervisors,
  KitchenStaff,
  AssemblyTechnicians
}

enum Status {
  Active,
  Terminated
}
// * Use a struct to store the employee type and whether they are
struct Employee {
  position : Position,
  status: Status
}
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn try_access(employeee: &Employee) -> Result<(), String> {
  match employeee.status {
    Status::Terminated => return Err("Terminated".to_owned()),
    _ => (),
  };

  match employeee.position {
    Position::Maintenance => Ok(()),
    Position::Marketing => Ok(()),
    Position::Managers => Ok(()),
    _ => Err("Invalid position".to_owned())
  }
}
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access(employeee: &Employee) -> Result<(), String> {
  let attempt_access = try_access(employeee)?;
  println!("Access OK");
  Ok(())
}

fn main() {
  let oscar = Employee {
    position: Position::LineSupervisors,
    status: Status::Active
  };

  match print_access(&oscar) {
    Err(e) => println!("Access denied: {:?}", e),
    _ => ()
  }
}
