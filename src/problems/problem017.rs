// TODO: optimize instead of naive sum

use crate::math::div_mod;

const UNITS: [u64; 20] = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
const TENS: [u64; 10] = [u64::MAX, 3, 6, 6, 5, 5, 5, 7, 6, 6];
const HUNDRED: u64 = 7;
const THOUSAND: u64 = 8;
const AND: u64 = 3;

pub fn subject() -> String {
    solve(1000).to_string()
}

fn solve(limit: usize) -> u64 {
    (1..=limit).map(count_letters).sum()
}

fn count_letters(n: usize) -> u64 {
    if n < 20 {
        return UNITS[n];
    } else if n < 100 {
        let (t, u) = div_mod(n, 10);
        return TENS[t] + UNITS[u];
    } else if n < 1_000 {
        let (h, u) = div_mod(n, 100);
        let mut res = UNITS[h] + HUNDRED;
        if u > 0 {
            res += AND + count_letters(u);
        }
        return res;
    } else if n < 1_000_000 {
        let (t, u) = div_mod(n, 1_000);
        let mut res = count_letters(t) + THOUSAND;
        if u > 0 {
            res += count_letters(u);
        }
        return res;
    }
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "21124");
    }

    #[test]
    fn test_count_letters() {
        assert_eq!(count_letters(115), 20);
        assert_eq!(count_letters(342), 23);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(0), 0);
        assert_eq!(solve(5), 19);
        assert_eq!(solve(1000), 21124);
    }
}
