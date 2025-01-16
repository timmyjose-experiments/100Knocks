// Employ enum to represent a type that can be one of many variants.
#![allow(dead_code)]

mod bad {
    const CIRCLE: u8 = 1;
    const RECTANGLE: u8 = 2;
    const TRIANGLE: u8 = 3;

    pub(crate) fn area(shape_type: u8, dimensions: &[f64]) -> f64 {
        match shape_type {
            1 => std::f64::consts::PI * dimensions[0] * dimensions[0],
            2 => dimensions[0] * dimensions[1],
            3 => {
                let s = (dimensions[0] + dimensions[1] + dimensions[2]) / 2.0;
                (s * (s - dimensions[0]) * (s - dimensions[1]) * (s - dimensions[2])).sqrt()
            }
            _ => 0.0,
        }
    }
}

mod good {
    pub(crate) enum Shape {
        Circle { radius: f64 },
        Rectangle { length: f64, breadth: f64 },
        Triangle { a: f64, b: f64, c: f64 },
    }

    pub(crate) fn area(shape: Shape) -> f64 {
        use Shape::*;
        match shape {
            Circle { radius } => std::f64::consts::PI * radius * radius,
            Rectangle { length, breadth } => length * breadth,
            Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_bad() {
        assert_eq!(bad::area(1, &[10.0]), 314.1592653589793);
        assert_eq!(bad::area(2, &[10.0, 20.0]), 200.0);
        assert_eq!(bad::area(3, &[3.0, 4.0, 5.0]), 6.0);
    }

    #[test]
    fn test_area_good() {
        assert_eq!(
            good::area(good::Shape::Circle { radius: 10.0 }),
            314.1592653589793
        );
        assert_eq!(
            good::area(good::Shape::Rectangle {
                length: 20.0,
                breadth: 10.0
            }),
            200.0
        );
        assert_eq!(
            good::area(good::Shape::Triangle {
                a: 3.0,
                b: 4.0,
                c: 5.0
            }),
            6.0
        );
    }
}
