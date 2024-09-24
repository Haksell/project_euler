pub fn is_leap(y: u64) -> bool {
    y % 400 == 0 || (y % 4 == 0 && y % 100 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap() {
        assert!(is_leap(1600));
        assert!(!is_leap(1900));
        assert!(is_leap(2000));
        assert!(!is_leap(2023));
        assert!(is_leap(2024));
        assert!(!is_leap(2100));
        assert!(is_leap(2400));
    }
}
