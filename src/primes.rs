use std::collections::HashMap;

use itertools::Itertools;

use crate::math::isqrt;

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

pub fn divisors_below(n: u64) -> Vec<HashMap<u64, u64>> {
    let mut nums = (0..n as u64).collect_vec();
    let mut divisors = vec![HashMap::new(); n as usize];
    for i in 2..=isqrt(n) {
        if nums[i as usize] == i as u64 {
            for j in (i * i..n).step_by(i as usize) {
                let mut count = 1;
                nums[j as usize] /= i;
                while nums[j as usize] % i == 0 {
                    count += 1;
                    nums[j as usize] /= i;
                }
                divisors[j as usize].insert(i, count);
            }
        }
    }
    for (i, &n) in nums.iter().enumerate() {
        if n > 1 {
            divisors[i].insert(n, 1);
        }
    }
    divisors
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
    fn test_divisors_below() {
        assert_eq!(
            divisors_below(11),
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
}
