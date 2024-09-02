pub fn subject() -> String {
    solve(100).to_string()
}

fn solve(n: u64) -> u64 {
    let square_of_sum = (n * (n + 1) / 2).pow(2);
    let sum_of_squares = n * (n + 1) * (2 * n + 1) / 6;
    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "25164150");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(4), 70);
        assert_eq!(solve(10), 2640);
    }
}
