use crate::primes::primes_below;

pub fn subject() -> String {
    solve(10_001).to_string()
}

fn solve(nth: usize) -> u64 {
    assert!(nth > 0);
    let mut limit = 1 << 6;
    loop {
        let primes = primes_below(limit);
        if primes.len() > nth {
            return primes[nth - 1];
        }
        limit <<= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(1), 2);
        assert_eq!(solve(2), 3);
        assert_eq!(solve(3), 5);
        assert_eq!(solve(4), 7);
        assert_eq!(solve(5), 11);
        assert_eq!(solve(6), 13);
        assert_eq!(solve(7), 17);
        assert_eq!(solve(8), 19);
        assert_eq!(solve(9), 23);
        assert_eq!(solve(10), 29);
        assert_eq!(solve(25), 97);
        assert_eq!(solve(168), 997);
    }
}
