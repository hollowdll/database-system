/// Checks if input contains whitespace characters.
/// 
/// Returns true if at least one whitespace is found.
pub fn has_whitespaces(input: &str) -> bool {
    for c in input.chars() {
        if c.is_whitespace() {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::util::has_whitespaces;

    #[test]
    fn test_has_whitespaces() {
        let input1 = "no";
        assert!(!has_whitespaces(input1));

        let input2 = "ye s";
        assert!(has_whitespaces(input2));
    }
}