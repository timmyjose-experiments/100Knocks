// Avoid abbreviations that are not universally understood
#![allow(dead_code)]

mod bad {
    pub(crate) fn calc_tp(ip: f64, tr: f64) -> f64 {
        let tp = ip + ip * tr;
        tp
    }
}

mod good {
    pub(crate) fn calculate_total_price(item_price: f64, tax_rate: f64) -> f64 {
        let total_price = item_price + item_price * tax_rate;
        total_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_tp_bad() {
        assert_eq!(bad::calc_tp(100.0, 0.1), 110.0);
    }

    #[test]
    fn test_calculate_total_price_good() {
        assert_eq!(good::calculate_total_price(100.0, 0.10), 110.0);
    }
}
