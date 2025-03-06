pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut digit_num = 0;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            digit_num += 1;
            if double {
                let double_digit = digit * 2;
                sum += if double_digit > 9 {
                    double_digit - 9
                } else {
                    double_digit
                };
            } else {
                sum += digit;
            }
            double = !double;
        } else if c.is_whitespace() {
            continue;
        } else {
          return false;
        }
    }

    digit_num >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_short_cc_number() {
        assert!(!luhn("1"));
        assert!(!luhn(""));
    }

    #[test]
    fn test_2_digit_cc_number() {
        assert!(luhn("00"));
    }

    #[test]
    fn test_invalid_char_cc_number() {
        assert!(!luhn("0!0"));
    }
}
