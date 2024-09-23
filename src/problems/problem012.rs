use crate::primes::count_divisors;

pub fn subject() -> String {
    solve(500).to_string()
}

fn solve(limit: u64) -> u64 {
    for i in 1.. {
        let triangle = (i * (i + 1)) >> 1;
        if count_divisors(triangle) > limit {
            return triangle;
        }
    }
    panic!("limit too big");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "76576500");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(0), 1);
        assert_eq!(solve(1), 3);
        assert_eq!(solve(2), 6);
        assert_eq!(solve(3), 6);
        assert_eq!(solve(4), 28);
        assert_eq!(solve(5), 28);
        assert_eq!(solve(6), 36);
        assert_eq!(solve(7), 36);
        assert_eq!(solve(8), 36);
        assert_eq!(solve(9), 120);
        assert_eq!(solve(10), 120);
        assert_eq!(solve(11), 120);
    }
}
