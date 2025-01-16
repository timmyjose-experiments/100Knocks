// Name modules and files to reflect their content and purpose
#![allow(dead_code)]

trait Describable {
    fn describe(&self) -> String;
}

mod bad {
    pub mod module1 {
        use crate::Describable;

        pub struct Data {
            pub id: i32,
            pub name: String,
        }

        impl Data {
            pub fn new(id: i32, name: &str) -> Self {
                Data {
                    id,
                    name: name.to_owned(),
                }
            }
        }

        impl Describable for Data {
            fn describe(&self) -> String {
                format!("id: {}, name: {}", self.id, self.name)
            }
        }
    }
}

mod good {
    pub mod user {
        use crate::Describable;

        pub struct User {
            pub id: i32,
            pub name: String,
        }

        impl User {
            pub fn new(id: i32, name: &str) -> Self {
                User {
                    id,
                    name: name.to_owned(),
                }
            }
        }

        impl Describable for User {
            fn describe(&self) -> String {
                format!("id: {}, name: {}", self.id, self.name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_bad() {
        let datum = bad::module1::Data::new(1, "Bob");
        assert_eq!(datum.describe(), "id: 1, name: Bob");
    }

    #[test]
    fn test_user_good() {
        let bob = good::user::User::new(1, "Bob");
        assert_eq!(bob.describe(), "id: 1, name: Bob");
    }
}
