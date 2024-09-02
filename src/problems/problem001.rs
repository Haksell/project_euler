use crate::math;
use cached::proc_macro::cached;
use itertools::Itertools as _;

pub fn subject() -> String {
    solve(&[3, 5], 1000).to_string()
}

fn naive(factors: &[u64], limit: u64) -> u64 {
    (1..limit)
        .filter(|&n| factors.iter().any(|&factor| factor > 0 && n % factor == 0))
        .sum()
}

fn solve(factors: &[u64], limit: u64) -> u64 {
    if limit <= 1 {
        return 0;
    }

    let factors = {
        let mut reduced = vec![];
        for &f in factors.iter().sorted() {
            if f != 0 && reduced.iter().all(|r| f % r != 0) {
                reduced.push(f);
            }
        }
        reduced
    };

    let naive_complexity = factors.len() * limit as usize;
    let smart_complexity = 1 << factors.len();
    if naive_complexity < smart_complexity {
        return naive(&factors, limit);
    }

    fn helper(factors: &[u64], limit: u64, lcm: u64, i: usize, mult: i64) -> i64 {
        if lcm >= limit {
            0
        } else if i == factors.len() {
            sum_multiples(lcm, limit) as i64 * mult
        } else {
            helper(factors, limit, lcm, i + 1, mult)
                + helper(
                    factors,
                    limit,
                    math::lcm(lcm, factors[i]),
                    i + 1,
                    if mult == 1 { -1 } else { 1 },
                )
        }
    }

    helper(&factors, limit, 1, 0, 0) as u64
}

#[cached]
fn sum_multiples(n: u64, limit: u64) -> u64 {
    let cnt = (limit - 1) / n;
    n * cnt * (cnt + 1) >> 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_subject() {
        assert_eq!(solve(&[3, 5], 1000), 233168);
    }

    #[test]
    fn test_basic() {
        assert_eq!(solve(&[4], 20), 40);
        assert_eq!(solve(&[4], 21), 60);
        assert_eq!(solve(&[3, 5], 10), 23);
        assert_eq!(solve(&[4, 5], 10), 17);
        assert_eq!(solve(&[3, 6], 10), 18);
        assert_eq!(solve(&[2, 3, 4], 10), 32);
        assert_eq!(solve(&[4, 5, 6], 21), 114);
        assert_eq!(solve(&[1, 2, 3, 4, 5, 42], 101), 5050);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(solve(&[], 10), 0);
        assert_eq!(solve(&[0], 10), 0);
        assert_eq!(solve(&[1], 10), 45);
        assert_eq!(solve(&[0, 1, 1, 1, 0], 10), 45);
        assert_eq!(solve(&[3, 5], 0), 0);
        assert_eq!(solve(&[3, 5], 1), 0);
        assert_eq!(solve(&[3, 5], 2), 0);
        assert_eq!(solve(&[3, 5], 3), 0);
        assert_eq!(solve(&[3, 5], 4), 3);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let num_factors = rng.gen_range(1..=5);
            let factors: Vec<u64> = (0..num_factors).map(|_| rng.gen_range(1..=12)).collect();
            let limit = rng.gen_range(10..=1000);
            assert_eq!(solve(&factors, limit), naive(&factors, limit));
        }
    }
}
