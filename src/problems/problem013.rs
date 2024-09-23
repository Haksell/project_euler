// TODO: use BigInt struct

use std::fs;

pub fn subject() -> String {
    let nums = parse_file("input/013.txt");
    solve(&nums, 10)
        .iter()
        .map(|&d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

fn parse_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            assert!(line.chars().all(|c| c.is_digit(10)));
            line.to_string()
        })
        .collect()
}

fn solve(nums: &[String], first_digits: usize) -> Vec<u8> {
    let max_len = nums.iter().map(|n| n.len()).max().unwrap();
    let mut digits = vec![0u64; max_len];
    for num in nums {
        for (i, d) in num.chars().rev().enumerate() {
            digits[i] += d as u64 - 48;
        }
    }

    let mut carry = 0;
    for i in 0..digits.len() {
        digits[i] += carry;
        carry = digits[i] / 10;
        digits[i] %= 10;
    }

    while carry > 0 {
        digits.push(carry % 10);
        carry /= 10;
    }

    digits
        .iter()
        .rev()
        .take(first_digits)
        .map(|&d| d as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "5537376230");
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(&["12345".to_string(), "777".to_string()], 3),
            vec![1, 3, 1]
        );
        assert_eq!(
            solve(&["12345".to_string(), "777".to_string()], 6),
            vec![1, 3, 1, 2, 2]
        );
    }
}
