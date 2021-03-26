/// Validates the given number for Luhn Algorithm.
/// Ref: https://en.wikipedia.org/wiki/Luhn_algorithm#:~:text=The%20Luhn%20algorithm%20or%20Luhn,Provider%20Identifier%20numbers%20in%20the
///
/// ## Example Usage
/// ```rust
/// use checkluhn::validate;
///
/// fn main() {
/// let n = "4111111111111111";
///     assert!(validate(n))
/// }
/// ```
pub fn validate(value: &str) -> bool {
    let mut chars: Vec<char> = value
        .chars()
        .collect();
    chars.reverse();
    let mut is_second = false;
    let mut sum = 0;

    for c in chars {
        assert!(c.is_digit(10));
        let mut d = c.to_digit(10).unwrap();
        if is_second {
            d *= 2;
        }
        sum += d / 10;
        sum += d % 10;
        is_second = !is_second;
    }

    sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn test_pass() {
        assert!(validate("6796265520244"));
        assert!(validate("4844161459546174"));
        assert!(validate("4035300539804083"));
        assert!(validate("378868637988407"));
        assert!(validate("4111111111111111"))
    }

    #[test]
    fn test_fail() {
        assert!(!validate("6796265520247"));
        assert!(!validate("4844161459546175"));
        assert!(!validate("4035300539804082"));
        assert!(!validate("378868637988406"));
    }
}