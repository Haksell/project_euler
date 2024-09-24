use crate::primes::sum_divisors_below;

pub fn subject() -> String {
    solve(10_000).to_string()
}

fn solve(limit: u64) -> u64 {
    let sum_divisors = sum_divisors_below(limit);
    let mut res = 0;
    for i in 10..limit {
        let sd = sum_divisors[i as usize] - i;
        if i < sd && sd < limit && sum_divisors[sd as usize] - sd == i {
            res += i + sd;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "31626");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(300), 504);
    }
}
