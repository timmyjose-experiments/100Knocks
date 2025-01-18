// Avoid using similar names for similar variables to prevent confusion
#![allow(dead_code)]

mod bad {
    pub(crate) struct UserDetails {
        pub detail1: String,
        pub detail2: u16,
        pub detail3: String,
    }

    pub(crate) fn user_details(user_details: UserDetails) -> String {
        format!(
            "Name: {}, age: {}, email: {}",
            user_details.detail1, user_details.detail2, user_details.detail3
        )
    }
}

mod good {
    pub(crate) struct UserDetails {
        pub name: String,
        pub age: u16,
        pub email: String,
    }

    pub(crate) fn user_details(user_details: UserDetails) -> String {
        format!(
            "Name: {}, age: {}, email: {}",
            user_details.name, user_details.age, user_details.email
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_details_bad() {
        let user = bad::UserDetails {
            detail1: "Bob".to_string(),
            detail2: 32,
            detail3: "bob@bobbers.com".to_string(),
        };

        assert_eq!(
            bad::user_details(user),
            "Name: Bob, age: 32, email: bob@bobbers.com"
        );
    }

    #[test]
    fn test_user_details_good() {
        let bob = good::UserDetails {
            name: "Bob".to_string(),
            age: 32,
            email: "bob@bobbers.com".to_string(),
        };

        assert_eq!(
            good::user_details(bob),
            "Name: Bob, age: 32, email: bob@bobbers.com"
        );
    }
}
