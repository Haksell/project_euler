use crate::bigint::BigInt;

pub fn subject() -> String {
    solve(100).to_string()
}

fn solve(n: u64) -> u64 {
    let mut res = BigInt::from(1);
    for i in 2..n {
        res *= i;
    }
    res.digits().iter().map(|&d| d as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "648");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 27);
    }
}
