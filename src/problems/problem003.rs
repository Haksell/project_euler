pub fn subject() -> String {
    solve(600_851_475_143).to_string()
}

fn solve(mut n: u64) -> u64 {
    assert!(n >= 2);
    let mut res = 2;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            n /= i;
            res = i;
        } else {
            i += 1;
        }
    }
    res.max(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "6857");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 2);
        assert_eq!(solve(3), 3);
        assert_eq!(solve(4), 2);
        assert_eq!(solve(5), 5);
        assert_eq!(solve(6), 3);
        assert_eq!(solve(7), 7);
        assert_eq!(solve(8), 2);
        assert_eq!(solve(9), 3);
        assert_eq!(solve(10), 5);
        assert_eq!(solve(11), 11);
        assert_eq!(solve(12), 3);
    }
}
