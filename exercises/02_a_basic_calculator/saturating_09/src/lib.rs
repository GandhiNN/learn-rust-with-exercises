pub fn factorial(n: u32) -> u32 {
    let mut result = 1u32;
    for i in 1..=n {
        // Use saturating multiplication to stop at the
        // maximum value of u32 rather than overflowing
        // and wrapping around
        result = result.saturating_mul(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }
}
