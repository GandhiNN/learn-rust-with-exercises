pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twentieth() {
        // 20! is 2432902008176640000 which is too large to fit in a u32
        // With the default dev profile, this will panic when we run `cargo test`
        // We want it to wrap around instead
        assert_eq!(factorial(20), 2_192_834_560);
    }
}
