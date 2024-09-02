use crate::primes::primes_below;

pub fn subject() -> String {
    solve(2_000_000).to_string()
}

fn solve(limit: u64) -> u64 {
    primes_below(limit).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "142913828922");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 0);
        assert_eq!(solve(10), 17);
        assert_eq!(solve(13), 28);
        assert_eq!(solve(14), 41);
    }
}
