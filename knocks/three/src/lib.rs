// use iter, map, and filter for Functional-style processing
#![allow(dead_code)]

mod bad {
    pub(crate) fn square_evens(ns: &[i32]) -> Vec<i32> {
        let mut res = Vec::new();

        for n in ns {
            if n % 2 == 0 {
                res.push(n * n);
            }
        }
        res
    }
}

mod good {
    pub(crate) fn square_evens(ns: &[i32]) -> Vec<i32> {
        ns.iter()
            .filter(|n| **n % 2 == 0)
            .map(|n| n * n)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_evens_bad() {
        let ns = (1..=10).collect::<Vec<_>>();
        assert_eq!(bad::square_evens(&ns), [4, 16, 36, 64, 100]);
    }

    #[test]
    fn test_square_evens_good() {
        let ns = (1..=10).collect::<Vec<_>>();
        assert_eq!(good::square_evens(&ns), [4, 16, 36, 64, 100]);
    }
}
