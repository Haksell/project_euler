use crate::math::choose;

pub fn subject() -> String {
    solve(20, 20).to_string()
}

fn solve(width: u64, height: u64) -> u64 {
    choose(width + height, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "137846528820");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(2, 2), 6);
        assert_eq!(solve(3, 3), 20);
        assert_eq!(solve(4, 4), 70);
        assert_eq!(solve(20, 2), 231);
        assert_eq!(solve(20, 20), 137846528820);
    }
}
