// TODO: use BigInt struct

use crate::bigint::BigInt;
use std::fs;

pub fn subject() -> String {
    let nums = parse_file("input/013.txt");
    solve(&nums, 10)
        .iter()
        .map(|&d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

fn parse_file(path: &str) -> Vec<BigInt> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(BigInt::from)
        .collect()
}

fn solve(nums: &[BigInt], first_digits: usize) -> Vec<u8> {
    nums.iter()
        .fold(BigInt::zero(), |mut acc, num| {
            acc += num.clone();
            acc
        })
        .digits()
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
        assert_eq!(solve(&["12345".into(), "777".into()], 3), vec![1, 3, 1]);
        assert_eq!(
            solve(&["12345".into(), "777".into()], 6),
            vec![1, 3, 1, 2, 2]
        );
    }
}
