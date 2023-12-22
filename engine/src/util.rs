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