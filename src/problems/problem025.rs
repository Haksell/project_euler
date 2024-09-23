// TODO: optimize log(phi), matrices or binary search
// log(10, phi) = 4.784971966781666

use crate::bigint::BigInt;

pub fn subject() -> String {
    solve(1000).to_string()
}

fn solve(digits: u64) -> u64 {
    if digits <= 1 {
        return digits;
    }
    let target = BigInt::pow(10, digits - 1);
    let mut a = BigInt::from(8);
    let mut b = BigInt::from(13);
    let mut i = 7;
    while b < target {
        a += &b;
        std::mem::swap(&mut a, &mut b);
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "4782");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(0), 0);
        assert_eq!(solve(1), 1);
        assert_eq!(solve(2), 7);
        assert_eq!(solve(3), 12);
        assert_eq!(solve(4), 17);
        assert_eq!(solve(1000), 4782);
    }
}
