use std::collections::HashMap;

pub fn subject() -> String {
    solve(500).to_string()
}

fn solve(limit: u64) -> u64 {
    let mut primes: Vec<u64> = vec![];
    let mut factors = vec![HashMap::<u64, u64>::new(); 2];

    for n in 2.. {
        let smallest_factor = {
            let mut smallest_factor = n;
            for &p in &primes {
                if p * p > n {
                    break;
                }
                if n % p == 0 {
                    smallest_factor = p;
                    break;
                }
            }
            smallest_factor
        };

        if smallest_factor == n {
            primes.push(n);
            factors.push(HashMap::from([(n, 1)]));
        } else {
            let mut factors_n = factors[(n / smallest_factor) as usize].clone();
            *factors_n.entry(smallest_factor).or_insert(0) += 1;
            factors.push(factors_n);
        }

        let mut factors_triangle = factors[n as usize - 1].clone();
        for (&p, &c) in &factors[n as usize] {
            *factors_triangle.entry(p).or_insert(0) += c;
        }
        *factors_triangle.get_mut(&2).unwrap() -= 1;

        let triangle_divisors: u64 = factors_triangle.values().map(|&c| c + 1).product();
        if triangle_divisors > limit {
            return n * (n - 1) >> 1;
        }
    }

    panic!("limit too big");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "76576500");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(0), 1);
        assert_eq!(solve(1), 3);
        assert_eq!(solve(2), 6);
        assert_eq!(solve(3), 6);
        assert_eq!(solve(4), 28);
        assert_eq!(solve(5), 28);
        assert_eq!(solve(6), 36);
        assert_eq!(solve(7), 36);
        assert_eq!(solve(8), 36);
        assert_eq!(solve(9), 120);
        assert_eq!(solve(10), 120);
        assert_eq!(solve(11), 120);
    }
}
