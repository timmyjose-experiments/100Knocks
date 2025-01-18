// Use consistent naming conventions throughout the codebase
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bad {
    pub(crate) struct user {
        FirstName: String,
        LastName: String,
        Email: String,
    }

    impl user {
        pub fn createUser(first: &str, last: &str, email: &str) -> Self {
            user {
                FirstName: first.to_owned(),
                LastName: last.to_owned(),
                Email: email.to_owned(),
            }
        }

        pub fn getFullName(&self) -> String {
            format!("{} {}", self.FirstName, self.LastName)
        }
    }
}

mod good {
    pub(crate) struct User {
        first_name: String,
        last_name: String,
        email: String,
    }

    impl User {
        pub fn create_user(first_name: &str, last_name: &str, email: &str) -> Self {
            User {
                first_name: first_name.to_owned(),
                last_name: last_name.to_owned(),
                email: email.to_owned(),
            }
        }

        pub fn get_full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getFullName_bad() {
        let bob = bad::user::createUser("Bob", "Bobbers", "bob@bobbers.com");
        assert_eq!(bob.getFullName(), "Bob Bobbers");
    }

    #[test]
    fn test_get_full_name_good() {
        let bob = good::User::create_user("Bob", "Bobbers", "bob@bobbers.com");
        assert_eq!(bob.get_full_name(), "Bob Bobbers");
    }
}
