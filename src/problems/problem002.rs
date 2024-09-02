pub fn subject() -> String {
    solve(4_000_000).to_string()
}

fn solve(limit: u64) -> u64 {
    let mut a = 2;
    let mut b = 3;
    let mut res = 0;
    while a <= limit {
        res += a;
        a += b << 1;
        b = (a << 1) - b;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(solve(4_000_000), 4613732);
    }

    #[test]
    fn test_small() {
        assert_eq!(solve(0), 0);
        assert_eq!(solve(1), 0);
        assert_eq!(solve(2), 2);
        assert_eq!(solve(7), 2);
        assert_eq!(solve(8), 10);
        assert_eq!(solve(9), 10);
        assert_eq!(solve(100), 44);
        assert_eq!(solve(150), 188);
    }
}
