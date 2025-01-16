// Use trait to define shared behaviour across different types
#![allow(dead_code)]

mod bad {
    pub(crate) struct Dog {
        name: String,
        age: u8,
    }

    impl Dog {
        pub fn new(name: &str, age: u8) -> Self {
            Dog {
                name: name.to_owned(),
                age,
            }
        }

        pub fn describe(&self) -> String {
            format!("Dog named {} is {} years old", self.name, self.age)
        }
    }

    pub(crate) struct Car {
        model: String,
        year: u16,
    }

    impl Car {
        pub fn new(model: &str, year: u16) -> Self {
            Car {
                model: model.to_owned(),
                year,
            }
        }

        pub fn describe(&self) -> String {
            format!("Car with model {}, year {}", self.model, self.year)
        }
    }
}

mod good {
    pub(crate) trait Describable {
        fn describe(&self) -> String;
    }

    pub(crate) struct Dog {
        name: String,
        age: u8,
    }

    impl Dog {
        pub fn new(name: &str, age: u8) -> Self {
            Dog {
                name: name.to_owned(),
                age,
            }
        }
    }

    impl Describable for Dog {
        fn describe(&self) -> String {
            format!("Dog named {} is {} years old", self.name, self.age)
        }
    }

    pub(crate) struct Car {
        model: String,
        year: u16,
    }

    impl Car {
        pub fn new(model: &str, year: u16) -> Self {
            Car {
                model: model.to_owned(),
                year,
            }
        }
    }

    impl Describable for Car {
        fn describe(&self) -> String {
            format!("Car with model {}, year {}", self.model, self.year)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_bad() {
        let dog = bad::Dog::new("Rufus", 16);
        assert_eq!(dog.describe().as_str(), "Dog named Rufus is 16 years old");

        let car = bad::Car::new("Lexus", 1997);
        assert_eq!(car.describe().as_str(), "Car with model Lexus, year 1997");
    }

    #[test]
    fn test_describe_good() {
        use good::Describable;

        let dog = good::Dog::new("Rufus", 16);
        assert_eq!(dog.describe().as_str(), "Dog named Rufus is 16 years old");

        let car = good::Car::new("Lexus", 1997);
        assert_eq!(car.describe().as_str(), "Car with model Lexus, year 1997");
    }
}
