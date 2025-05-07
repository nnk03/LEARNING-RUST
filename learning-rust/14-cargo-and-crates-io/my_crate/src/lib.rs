//! # My Crate
//!
//! `my_crate` is a collection of utilities ...

/// Adds one to the number given
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
///assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(4), 5);
    }
}
