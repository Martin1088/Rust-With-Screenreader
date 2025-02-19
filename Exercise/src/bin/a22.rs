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
    match b {
        0 => None,
        _ => Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn middle_clamp() {
        let result = clamp(5, 1, 10);
        assert_eq!(result, 5, "perfect");
    }
    #[test]
    fn low_clamp() {
        let result = clamp(2, 5, 18);
        assert_eq!(result, 5, "wrong");
    }
    #[test]
    fn high_clamp() {
        let result = clamp(30, 5, 20);
        assert_eq!(result, 20, "wrong");
    }
    #[test]
    fn test_div() {
        let result = div(33, 3);
        assert_eq!(result, Some(11), "wrong");
    }
    #[test]
    fn div_null() {
        let result = div(22, 0);
        assert_eq!(result, None, "wrong" );
    }
    #[test]
    fn test_concat() {
        let result = concat("Merlin", "schläft");
        assert_eq!(result, "Merlinschläft", "wrong");
    }

}
