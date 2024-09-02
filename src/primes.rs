use crate::math::isqrt;

pub fn primes_below(n: u64) -> Vec<u64> {
    let mut sieve = vec![true; n as usize];
    sieve[0] = false;
    sieve[1] = false;
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
        .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
        .collect()
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
}
