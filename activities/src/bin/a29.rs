// Topic: Generics & Functions
//
// Requirements:
// * Create a function that accepts the Priority trait as a generic parameter
//   * The function should print out the guest and their priority
// * Use the function with at least two different guests
//
// Notes:
// * Use the debug token "{:?}" to print out the information
// * Use the compiler to guide you to the correct generic constraints needed

#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}


#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

fn print_guest_priority<T: Priority + std::fmt::Debug>(guest: T) {
    println!("{:?} - {:?}", guest, guest.get_priority())
}

fn main() {
    print_guest_priority(ImportantGuest);
    print_guest_priority(Guest)
}
