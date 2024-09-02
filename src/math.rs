// TODO: https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u32, b: u32) -> u64 {
    a as u64 * b as u64 / gcd(a, b) as u64
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
}
