#![allow(dead_code)]
// Leverage Option and Resuly types for safe and explicit error handling

mod good {
    pub(crate) fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
        if y == 0.0 {
            Err("Cannot divide by zero")
        } else {
            Ok(x / y)
        }
    }
}

mod bad {
    pub(crate) fn divide(x: f64, y: f64) -> f64 {
        if y == 0.0 {
            panic!("Cannot divide by zero");
        }
        x / y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_bad() {
        assert_eq!(bad::divide(12.0, 2.0), 6.0);
    }

    #[test]
    #[should_panic]
    fn test_divide_bad_panic() {
        bad::divide(12.0, 0.0);
    }

    #[test]
    fn test_divide_good() {
        assert_eq!(good::divide(12.0, 2.0), Ok(6.0));
        assert_eq!(good::divide(12.0, 0.0), Err("Cannot divide by zero"));
    }
}
