#![allow(dead_code)]

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn width(&self) -> bool {
        self.width > 0
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn find_area_of_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
