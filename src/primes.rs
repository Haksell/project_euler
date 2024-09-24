#![allow(unused)] // TODO: remove

use crate::math::isqrt;
use itertools::Itertools;
use std::collections::HashMap;

pub fn primes_below(n: u64) -> Vec<u64> {
    let mut sieve = vec![true; n as usize];
    for i in 2..=isqrt(n) as usize {
        if sieve[i] {
            for j in (i * i..n as usize).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
        .collect()
}

pub fn get_factors(mut n: u64) -> HashMap<u64, u64> {
    let mut factors = HashMap::new();
    let mut d = 2;
    while d * d <= n {
        let mut count = 0;
        while n % d == 0 {
            count += 1;
            n /= d;
        }
        if count > 0 {
            factors.insert(d, count);
        }
        d += 1;
    }
    if n > 1 {
        factors.insert(n, 1);
    }
    factors
}

pub fn count_divisors(mut n: u64) -> u64 {
    let mut divisors = 1;
    let mut d = 2;
    while d * d <= n {
        let mut count = 0;
        while n % d == 0 {
            count += 1;
            n /= d;
        }
        divisors *= count + 1;
        d += 1;
    }
    if n > 1 {
        divisors <<= 1;
    }
    divisors
}

pub fn factors_below(n: u64) -> Vec<HashMap<u64, u64>> {
    let mut nums = (0..n as u64).collect_vec();
    let mut factors = vec![HashMap::new(); n as usize];
    for i in 2..=isqrt(n) {
        if nums[i as usize] == i as u64 {
            for j in (i * i..n).step_by(i as usize) {
                let mut count = 1;
                nums[j as usize] /= i;
                while nums[j as usize] % i == 0 {
                    count += 1;
                    nums[j as usize] /= i;
                }
                factors[j as usize].insert(i, count);
            }
        }
    }
    for (i, &n) in nums.iter().enumerate() {
        if n > 1 {
            factors[i].insert(n, 1);
        }
    }
    factors
}

pub fn sum_divisors_below(n: u64) -> Vec<u64> {
    let factors = factors_below(n);
    let mut sum_divisors = factors
        .iter()
        .map(|h| {
            h.iter()
                .map(|(&k, &v)| (k.pow(v as u32 + 1) - 1) / (k - 1))
                .product::<u64>()
        })
        .collect::<Vec<_>>();
    sum_divisors[0] = 0;
    sum_divisors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_below() {
        assert_eq!(primes_below(2), vec![]);
        assert_eq!(primes_below(7), vec![2, 3, 5]);
        assert_eq!(primes_below(8), vec![2, 3, 5, 7]);
        assert_eq!(primes_below(25), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
    }

    #[test]
    fn test_factors_below() {
        assert_eq!(
            factors_below(11),
            vec![
                HashMap::new(),
                HashMap::new(),
                HashMap::from([(2, 1)]),
                HashMap::from([(3, 1)]),
                HashMap::from([(2, 2)]),
                HashMap::from([(5, 1)]),
                HashMap::from([(2, 1), (3, 1)]),
                HashMap::from([(7, 1)]),
                HashMap::from([(2, 3)]),
                HashMap::from([(3, 2)]),
                HashMap::from([(2, 1), (5, 1)]),
            ]
        );
    }

    #[test]
    fn test_sum_divisors_below() {
        assert_eq!(
            sum_divisors_below(20),
            vec![0, 1, 3, 4, 7, 6, 12, 8, 15, 13, 18, 12, 28, 14, 24, 24, 31, 18, 39, 20]
        );
    }
}
