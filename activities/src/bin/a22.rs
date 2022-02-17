// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
/// lower - n - upper
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {
    let a = clamp(10, 100, 1000);
    println!("{:?}", a);

    let b = clamp(10, 10, 10);
    println!("{:?}", b);

    let c = clamp(100, 10, 10);
    println!("{:?}", c);

    let d = div(10, 2);
    println!("{:?}", d);

    let e = concat("first", "second");
    println!("{:?}", e);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_lower() {
        let result = clamp(10, 100, 1000);
        let expected = 100;
        assert_eq!(result, expected, "Should be 100");
    }

    #[test]
    fn check_clamp() {
        let result = clamp(10, 10, 10);
        let expected = 10;
        assert_eq!(result, expected, "Should be 10");
    }

    #[test]
    fn clamp_upper() {
        let result = clamp(100, 10, 10);
        let expected = 10;
        assert_eq!(result, expected, "Should be 100");
    }

    #[test]
    fn check_div() {
        let result = div(10, 2);
        let expected = Some(5);
        assert_eq!(result, expected, "Should be Some(5)");
    }

    #[test]
    fn check_concat() {
        let result = concat("first", "second");
        let expected = "first second";
        assert_eq!(result, expected, "Should be first second");
    }
}