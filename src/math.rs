use std::{cmp::min, mem::swap};

// https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    let tz_a = a.trailing_zeros();
    let tz_b = b.trailing_zeros();
    let tz_max = min(tz_a, tz_b);
    a >>= tz_a;
    b >>= tz_b;

    loop {
        if a == b {
            return a << tz_max;
        }
        if a > b {
            swap(&mut a, &mut b);
        }
        b -= a;
        b >>= b.trailing_zeros();
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn digits(mut n: u64) -> Vec<u64> {
    let mut res = vec![];
    while n >= 10 {
        res.push(n % 10);
        n /= 10;
    }
    res.push(n);
    res.reverse();
    res
}

pub fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut prev = 0;
    let mut guess = n;
    loop {
        let new_guess = (guess + n / guess) >> 1;
        if new_guess == prev {
            return guess;
        }
        prev = guess;
        guess = new_guess;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(0, 2), 2);
        assert_eq!(gcd(0, 3), 3);
        assert_eq!(gcd(0, 4), 4);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(0, 6), 6);

        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(1, 3), 1);
        assert_eq!(gcd(1, 4), 1);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(1, 6), 1);

        assert_eq!(gcd(2, 0), 2);
        assert_eq!(gcd(2, 1), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(2, 5), 1);
        assert_eq!(gcd(2, 6), 2);

        assert_eq!(gcd(3, 0), 3);
        assert_eq!(gcd(3, 1), 1);
        assert_eq!(gcd(3, 2), 1);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(3, 4), 1);
        assert_eq!(gcd(3, 5), 1);
        assert_eq!(gcd(3, 6), 3);

        assert_eq!(gcd(4, 0), 4);
        assert_eq!(gcd(4, 1), 1);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(4, 3), 1);
        assert_eq!(gcd(4, 4), 4);
        assert_eq!(gcd(4, 5), 1);
        assert_eq!(gcd(4, 6), 2);

        assert_eq!(gcd(6, 0), 6);
        assert_eq!(gcd(6, 1), 1);
        assert_eq!(gcd(6, 2), 2);
        assert_eq!(gcd(6, 3), 3);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(6, 5), 1);
        assert_eq!(gcd(6, 6), 6);

        assert_eq!(gcd(945, 1008), 63);
        assert_eq!(gcd(945, 1071), 63);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(42, 0), 0);
        assert_eq!(lcm(42, 1), 42);
        assert_eq!(lcm(42, 2), 42);
        assert_eq!(lcm(42, 3), 42);
        assert_eq!(lcm(42, 4), 84);
        assert_eq!(lcm(42, 5), 210);
        assert_eq!(lcm(42, 63), 126);
    }

    #[test]
    fn test_digits() {
        assert_eq!(digits(0), vec![0]);
        assert_eq!(digits(6), vec![6]);
        assert_eq!(digits(9), vec![9]);
        assert_eq!(digits(10), vec![1, 0]);
        assert_eq!(digits(42), vec![4, 2]);
        assert_eq!(digits(99), vec![9, 9]);
        assert_eq!(digits(100), vec![1, 0, 0]);
        assert_eq!(digits(666), vec![6, 6, 6]);
        assert_eq!(digits(999), vec![9, 9, 9]);
    }

    #[test]
    fn test_isqrt() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(2), 1);
        assert_eq!(isqrt(3), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(5), 2);
        assert_eq!(isqrt(6), 2);
        assert_eq!(isqrt(7), 2);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(10), 3);
        assert_eq!(isqrt(11), 3);
        assert_eq!(isqrt(12), 3);
        assert_eq!(isqrt(13), 3);
        assert_eq!(isqrt(14), 3);
        assert_eq!(isqrt(15), 3);
        assert_eq!(isqrt(16), 4);
        assert_eq!(isqrt(17), 4);
        assert_eq!(isqrt(18), 4);
        assert_eq!(isqrt(19), 4);
        assert_eq!(isqrt(20), 4);
        assert_eq!(isqrt(99), 9);
        assert_eq!(isqrt(100), 10);
        assert_eq!(isqrt(9999), 99);
        assert_eq!(isqrt(10000), 100);
        assert_eq!(isqrt(999999), 999);
        assert_eq!(isqrt(1000000), 1000);
    }
}
