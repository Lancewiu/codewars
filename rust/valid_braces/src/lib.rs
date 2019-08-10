mod valid_braces;

#[cfg(test)]
mod tests {
    use crate::valid_braces::valid_braces;
    #[test]
    fn basic_tests() {
        assert!(valid_braces("()"));
        assert!(!valid_braces("[(])"));
    }
}
