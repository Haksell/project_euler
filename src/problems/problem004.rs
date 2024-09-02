use crate::{math::digits, utils::is_palindrome};

pub fn subject() -> String {
    solve(3).to_string()
}

fn solve(num_digits: u8) -> u64 {
    assert!(num_digits > 0 && num_digits < 10);
    let mini = 10u64.pow(num_digits as u32 - 1);
    let maxi = 10u64.pow(num_digits as u32) - 1;
    let mut res = 0;
    for m1 in (mini..=maxi).rev() {
        if m1 * maxi <= res {
            break;
        }
        for m2 in (mini..=maxi).rev() {
            let prod = m1 * m2;
            if prod <= res {
                break;
            }
            if is_palindrome(&digits(prod)) {
                res = prod;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "906609");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(1), 9);
        assert_eq!(solve(2), 9009);
        assert_eq!(solve(3), 906609);
    }
}
