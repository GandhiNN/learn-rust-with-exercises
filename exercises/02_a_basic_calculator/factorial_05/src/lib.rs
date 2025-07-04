#![allow(unused)]
/// Given a non-negative integer, the factorial of `n`
/// is defined as the product of all positive integers
/// up to `n`.
fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fourth() {
        assert_eq!(factorial(5), 120);
    }
}
