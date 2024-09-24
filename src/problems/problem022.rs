use std::fs;

pub fn subject() -> String {
    let names = fs::read_to_string("input/022.txt")
        .unwrap()
        .split(',')
        .map(|s| s[1..s.len() - 1].to_string())
        .collect::<Vec<_>>();
    solve(names).to_string()
}

fn solve(mut names: Vec<String>) -> u64 {
    names.sort();
    names
        .iter()
        .enumerate()
        .map(|(i, s)| (i as u64 + 1) * s.chars().map(|c| c as u64 & 31).sum::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "871198282");
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(vec!["MDR".into(), "XD".into(), "LOL".into()]),
            1 * (12 + 15 + 12) + 2 * (13 + 4 + 18) + 3 * (24 + 4)
        );
    }
}
