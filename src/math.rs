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

// TODO: test all the functions
