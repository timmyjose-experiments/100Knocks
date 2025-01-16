// Prefix boolean values with is_, has_, or can_ to indicate their nature
#![allow(dead_code)]

mod bad {
    pub(crate) fn even(n: i32) -> bool {
        n % 2 == 0
    }

    pub(crate) fn permission(user_role: &str) -> bool {
        user_role == "admin"
    }

    pub(crate) fn drive(age: u16) -> bool {
        age >= 18
    }
}

mod good {
    pub(crate) fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    pub(crate) fn has_permission(user_role: &str) -> bool {
        user_role == "admin"
    }

    pub(crate) fn can_drive(age: u16) -> bool {
        age >= 18
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_bad() {
        assert!(bad::even(2));
        assert!(!bad::even(3));

        assert!(bad::permission("admin"));
        assert!(!bad::permission("non-admin"));

        assert!(bad::drive(18));
        assert!(!bad::drive(15));
    }

    #[test]
    fn test_is_even_good() {
        assert!(good::is_even(2));
        assert!(!good::is_even(3));

        assert!(good::has_permission("admin"));
        assert!(!good::has_permission("non-admin"));

        assert!(good::can_drive(18));
        assert!(!good::can_drive(15));
    }
}
