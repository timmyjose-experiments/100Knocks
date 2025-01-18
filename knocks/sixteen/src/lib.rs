// Use impl blocks to group related methods for a type
#![allow(dead_code)]

mod bad {
    pub(crate) struct Rectangle {
        width: u32,
        height: u32,
    }

    pub(crate) fn create_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub(crate) fn rectangle_area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }

    pub(crate) fn rectangle_can_hold(rect: &Rectangle, other_rect: &Rectangle) -> bool {
        rect.width >= other_rect.width && rect.height >= other_rect.height
    }
}

mod good {
    pub(crate) struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn create_rectangle(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_bad() {
        let r1 = bad::create_rectangle(10, 20);
        let r2 = bad::create_rectangle(20, 10);
        let r3 = bad::create_rectangle(10, 19);

        assert!(bad::rectangle_can_hold(&r1, &r3));
        assert!(!bad::rectangle_can_hold(&r1, &r2));

        assert_eq!(bad::rectangle_area(&r1), 200);
        assert_eq!(bad::rectangle_area(&r2), 200);
        assert_eq!(bad::rectangle_area(&r3), 190);
    }

    #[test]
    fn test_rectangle_good() {
        let r1 = good::Rectangle::create_rectangle(10, 20);
        let r2 = good::Rectangle::create_rectangle(20, 10);
        let r3 = good::Rectangle::create_rectangle(10, 19);

        assert!(r1.can_hold(&r3));
        assert!(!r1.can_hold(&r2));

        assert_eq!(r1.area(), 200);
        assert_eq!(r2.area(), 200);
        assert_eq!(r3.area(), 190);
    }
}
