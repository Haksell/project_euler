use crate::math::lcm;

pub fn subject() -> String {
    solve(20).to_string()
}

fn solve(limit: u64) -> u64 {
    assert!(limit > 0);
    (2..=limit).fold(1, |acc, i| lcm(acc, i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "232792560");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(1), 1);
        assert_eq!(solve(2), 2);
        assert_eq!(solve(3), 6);
        assert_eq!(solve(4), 12);
        assert_eq!(solve(5), 60);
        assert_eq!(solve(6), 60);
        assert_eq!(solve(7), 420);
        assert_eq!(solve(8), 840);
        assert_eq!(solve(9), 2520);
        assert_eq!(solve(10), 2520);
    }
}
