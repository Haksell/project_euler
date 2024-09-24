use super::duplicates::pyramid_problem;

pub fn subject() -> String {
    pyramid_problem("input/067.txt").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "7273");
    }
}
