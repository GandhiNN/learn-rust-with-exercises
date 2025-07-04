pub fn intro() -> &'static str {
    "I'm ready to start modelling a software ticket!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to start modelling a software ticket!");
    }
}
