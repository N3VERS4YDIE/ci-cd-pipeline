/// A simple calculator module to demonstrate CI/CD
pub mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}

fn main() {
    let a = 10;
    let b = 5;

    println!("\nCalculator Operations:");
    println!("{} + {} = {}", a, b, calculator::add(a, b));
    println!("{} - {} = {}", a, b, calculator::subtract(a, b));
    println!("{} * {} = {}", a, b, calculator::multiply(a, b));

    match calculator::divide(a, b) {
        Ok(result) => println!("{} / {} = {}", a, b, result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n✅ All operations completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::calculator::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 5), -5);
        assert_eq!(subtract(10, 10), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(9, 3), Ok(3));
        assert_eq!(divide(7, 2), Ok(3)); // Integer division
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
        assert_eq!(divide(5, 0), Err("Cannot divide by zero".to_string()));
    }
}
