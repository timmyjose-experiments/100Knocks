#![allow(dead_code)]

// Use pattern matching for clear and concise control flow

enum Direction {
    North,
    South,
    East,
    West,
}

mod bad {
    use crate::Direction;

    pub(crate) fn get_direction_name(direction: Direction) -> &'static str {
        if let Direction::North = direction {
            "North"
        } else if let Direction::South = direction {
            "South"
        } else if let Direction::East = direction {
            "East"
        } else {
            "West"
        }
    }
}

mod good {
    use crate::Direction;

    pub(crate) fn get_direction_name(direction: Direction) -> &'static str {
        match direction {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_direction_name_bad() {
        use bad::get_direction_name;

        assert_eq!("North", get_direction_name(Direction::North));
        assert_eq!("South", get_direction_name(Direction::South));
        assert_eq!("East", get_direction_name(Direction::East));
        assert_eq!("West", get_direction_name(Direction::West));
    }

    #[test]
    fn test_get_direction_name_good() {
        use good::get_direction_name;

        assert_eq!("North", get_direction_name(Direction::North));
        assert_eq!("South", get_direction_name(Direction::South));
        assert_eq!("East", get_direction_name(Direction::East));
        assert_eq!("West", get_direction_name(Direction::West));
    }
}
