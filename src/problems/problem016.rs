// TODO: use BigInt struct

pub fn subject() -> String {
    solve(1000).to_string()
}

fn solve(exponent: u64) -> u64 {
    let mut digits = vec![1u8];
    for _ in 0..exponent {
        let mut carry = 0;
        for i in 0..digits.len() {
            (digits[i], carry) = if digits[i] >= 5 {
                (digits[i] * 2 - 10 + carry, 1)
            } else {
                (digits[i] * 2 + carry, 0)
            };
        }
        if carry == 1 {
            digits.push(1);
        }
    }
    digits.iter().map(|&d| d as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "1366");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(0), 1);
        assert_eq!(solve(1), 2);
        assert_eq!(solve(2), 4);
        assert_eq!(solve(3), 8);
        assert_eq!(solve(4), 7);
        assert_eq!(solve(5), 5);
        assert_eq!(solve(6), 10);
        assert_eq!(solve(7), 11);
        assert_eq!(solve(8), 13);
        assert_eq!(solve(9), 8);
        assert_eq!(solve(10), 7);
        assert_eq!(solve(15), 26);
    }
}
