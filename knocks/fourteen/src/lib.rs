// Ensure function names clealry describe their actions or results
#![allow(dead_code)]

mod bad {
    pub(crate) fn calc(r: f64) -> f64 {
        std::f64::consts::PI * r * r
    }

    pub(crate) fn pg(s: &str) -> String {
        format!("Hello, {s}")
    }
}

mod good {
    pub(crate) fn calculate_circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }

    pub(crate) fn print_greeting(name: &str) -> String {
        format!("Hello, {name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_bad() {
        assert_eq!(bad::calc(10.0), 314.1592653589793);
    }

    #[test]
    fn test_pg_bad() {
        assert_eq!(bad::pg("Bob"), "Hello, Bob");
    }

    #[test]
    fn test_calculate_circle_area_good() {
        assert_eq!(good::calculate_circle_area(10.0), 314.1592653589793);
    }

    #[test]
    fn test_print_greeting_good() {
        assert_eq!(good::print_greeting("Bob"), "Hello, Bob");
    }
}
