// Name variables and functions decriptively to convey their purpose
#![allow(dead_code)]

mod bad {
    pub(crate) fn calc(w: f64, h: f64) -> f64 {
        w * h
    }
}

mod good {
    pub(crate) fn area_of_rectangle(width: f64, height: f64) -> f64 {
        width * height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_bad() {
        assert_eq!(bad::calc(20.0, 10.0), 200.0);
    }

    #[test]
    fn test_area_of_rectangle_good() {
        assert_eq!(good::area_of_rectangle(20.0, 10.0), 200.0);
    }
}
