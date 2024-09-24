use std::{cmp::max, fs};

pub fn pyramid_problem(path: &str) -> u64 {
    find_max_path(parse_pyramid(path))
}

fn parse_pyramid(path: &str) -> Vec<Vec<u64>> {
    let pyramid = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert!(pyramid.len() > 0);
    assert!(pyramid
        .iter()
        .enumerate()
        .all(|(i, row)| i + 1 == row.len()));
    pyramid
}

fn find_max_path(mut grid: Vec<Vec<u64>>) -> u64 {
    for y in (0..grid.len() - 1).rev() {
        for x in 0..=y {
            grid[y][x] += max(grid[y + 1][x], grid[y + 1][x + 1]);
        }
    }
    grid[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_path() {
        assert_eq!(
            find_max_path(vec![vec![3], vec![7, 4], vec![2, 4, 6,], vec![8, 5, 9, 3]]),
            23
        );
    }
}
