use crate::combinatorics::nth_perm;

pub fn subject() -> String {
    nth_perm(&[0, 1, 2], 4)
        .unwrap()
        .into_iter()
        .map(u8::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "");
    }
}
