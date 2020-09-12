#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]

//! This is a logic gates simulation crate built to demonstrate writing unit tests and integreation tests

/// Implements a boolean `and` gate taking as input two bytes, represting bits, and returns a byte result
pub fn and(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) => 1,
        _ => 0,
    }
}

/// Implements a boolean `or` gate taking as input two bytes, represtings bits, and returning a byte result
pub fn or(a: u8, b: u8) -> u8 {
    match (a, b) {
        (0, 0) => 0,
        _ => 1,
    }
}

/// Implements a boolean `xor` gate taking as input two bytes, represtings bits, and returing a byte result
pub fn xor(a: u8, b: u8) -> u8 {
    // match (a, b) {
    //     (1, 0) | (0, 1) => 1,
    //     _ => 0
    // }
    if a != b {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{and, or, xor};

    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(0, 0));
    }

    #[test]
    fn test_or() {
        assert_eq!(1, or(1, 1));
        assert_eq!(1, or(0, 1));
        assert_eq!(1, or(1, 0));
        assert_eq!(0, or(0, 0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(0, 0));
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(0, 1));
    }
}
