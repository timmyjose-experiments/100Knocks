// Use full words instead of single letters for variable names
#![allow(dead_code)]

mod bad {
    pub(crate) fn calculate_area(l: f64, b: f64) -> f64 {
        l * b
    }
}

mod good {
    pub(crate) fn calculate_area(length: f64, breadth: f64) -> f64 {
        length * breadth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_area_bad() {
        assert_eq!(bad::calculate_area(20.0, 10.0), 200.0);
    }

    #[test]
    fn test_calculate_area_good() {
        assert_eq!(good::calculate_area(20.0, 10.0), 200.0);
    }
}
