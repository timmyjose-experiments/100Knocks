// Use snake_case for variable and function names for consistency
#![allow(dead_code)]
#![allow(non_snake_case)]

mod bad {
    pub(crate) fn calculateArea(rectangleWidth: f64, rectangleHeight: f64) -> f64 {
        rectangleWidth * rectangleHeight
    }
}

mod good {
    pub(crate) fn calculate_area(rectangle_width: f64, rectangle_height: f64) -> f64 {
        rectangle_width * rectangle_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_area_bad() {
        assert_eq!(bad::calculateArea(10.0, 20.0), 200.0);
    }

    #[test]
    fn test_calculate_area_good() {
        assert_eq!(good::calculate_area(10.0, 20.0), 200.0);
    }
}
