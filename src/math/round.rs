pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}


#[cfg(test)]
mod tests {
    use super::round;

    #[test]
    fn test_rounding_positive_number_to_int() {
        assert_eq!(round(10.99, 0), 11.0);
        assert_eq!(round(123.456, 0), 123.0);
    }

    #[test]
    fn test_rounding_negative_number_to_int() {
        assert_eq!(round(-10.99, 0), -11.0);
        assert_eq!(round(-123.456, 0), -123.0);
    }

    #[test]
    fn test_rounding_positive_number_with_decimals() {
        assert_eq!(round(3.14159, 2), 3.14);
        assert_eq!(round(1.2345678, 4), 1.2346);
    }

    #[test]
    fn test_rounding_negative_number_with_decimals() {
        assert_eq!(round(-3.14159, 2), -3.14);
        assert_eq!(round(-1.2345678, 4), -1.2346);
    }

    #[test]
    fn test_rounding_zero() {
        assert_eq!(round(0.0, 3), 0.0);
        assert_eq!(round(0.0, 0), 0.0);
    }

    #[test]
    fn test_rounding_large_numbers() {
        assert_eq!(round(99999.999, 0), 100000.0);
        assert_eq!(round(9876543.210987654, 6), 9876543.210988);
    }
}