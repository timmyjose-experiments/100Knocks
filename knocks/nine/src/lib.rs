// Use `const` and `static` for constants to make their immutability clear
#![allow(dead_code)]

mod bad {
    pub(crate) fn app_details() -> String {
        let max_users = 100;
        let app_name = "myapp";

        format!("{app_name}, {max_users}")
    }
}

mod good {
    const MAX_USERS: i32 = 100;
    static APP_NAME: &'static str = "myapp";

    pub(crate) fn app_details() -> String {
        format!("{APP_NAME}, {MAX_USERS}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_details_bad() {
        assert_eq!(bad::app_details(), "myapp, 100");
    }

    #[test]
    fn test_app_details_good() {
        assert_eq!(good::app_details(), "myapp, 100");
    }
}
