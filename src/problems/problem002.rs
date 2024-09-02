pub fn subject() -> String {
    solve(&[3, 5], 10).to_string()
}

fn solve(factors: &[u32], limit: u32) -> u32 {
    (1..limit)
        .filter(|&n| factors.iter().any(|&factor| n % factor == 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(&[3, 5], 10), 23);
        assert_eq!(solve(&[4, 5], 10), 17);
        assert_eq!(solve(&[3, 6], 10), 18);
        assert_eq!(solve(&[3, 5], 1000), 233168);
    }
}
