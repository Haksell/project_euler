pub fn subject() -> String {
    solve(1_000_000).to_string()
}

fn solve(limit: u64) -> u64 {
    let mut lengths = vec![None; limit as usize];
    lengths[1] = Some(1);
    let mut path = vec![];
    for mut n in 2..limit {
        while n >= limit || lengths[n as usize].is_none() {
            path.push(n);
            n = if n & 1 == 0 { n >> 1 } else { 3 * n + 1 };
        }
        let length = lengths[n as usize].unwrap();
        for (i, &n) in path.iter().rev().enumerate() {
            if n < limit {
                lengths[n as usize] = Some(length + i as u64 + 1);
            }
        }
        path.clear();
    }
    (1..limit)
        .max_by_key(|&i| lengths[i as usize].unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "837799");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(5), 3);
        assert_eq!(solve(10), 9);
        assert_eq!(solve(30), 27);
    }
}
