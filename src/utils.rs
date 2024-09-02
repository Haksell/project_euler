pub fn is_palindrome<T: PartialEq>(slice: &[T]) -> bool {
    (0..slice.len() >> 1).all(|i| slice[i] == slice[slice.len() - i - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome::<u64>(&[]));
        assert!(is_palindrome(&[42]));
        assert!(is_palindrome(&[42, 42]));
        assert!(is_palindrome(&[42, 42, 42]));
        assert!(is_palindrome(&[42, 0, 42]));
        assert!(is_palindrome(&[42, 0, 0, 42]));
        assert!(is_palindrome(&[42, 0, 17, 0, 42]));
        assert!(is_palindrome(&[42, 0, 17, 42, 42]));
        assert!(is_palindrome(&[42, 17, 17, 0, 42]));
        assert!(is_palindrome(&[42, 17, 42, 17]));
    }
}
