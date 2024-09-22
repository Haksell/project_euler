use std::{cmp::min, fs};

pub fn subject() -> String {
    solve(&parse_grid("input/011.txt"), 4).to_string()
}

fn parse_grid(path: &str) -> Vec<Vec<u64>> {
    let grid = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert!(grid.len() > 0);
    assert!(grid[0].len() > 0);
    assert!(grid.iter().all(|row| row.len() == grid[0].len()));
    grid
}

fn solve(grid: &Vec<Vec<u64>>, k: usize) -> u64 {
    lines(grid)
        .iter()
        .map(|line| solve_line(line, k))
        .max()
        .unwrap()
}

fn lines(grid: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let height = grid.len();
    let width = grid[0].len();
    grid.iter()
        .chain(
            &(0..width)
                .map(|x| (0..height).map(|y| grid[y][x]).collect::<Vec<_>>())
                .collect::<Vec<Vec<u64>>>(),
        )
        .chain(
            &(0..width)
                .map(|x| {
                    (0..min(height, width - x))
                        .map(|y| grid[y][x + y])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<u64>>>(),
        )
        .chain(
            &(1..height)
                .map(|y| {
                    (0..min(width, height - y))
                        .map(|x| grid[y + x][x])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<u64>>>(),
        )
        .chain(
            &(0..width)
                .map(|x| {
                    (0..min(height, width - x))
                        .map(|y| grid[y][width - 1 - x - y])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<u64>>>(),
        )
        .chain(
            &(1..height)
                .map(|y| {
                    (0..min(width, height - y))
                        .map(|x| grid[y + x][width - x - 1])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<u64>>>(),
        )
        .cloned()
        .collect()
}

fn solve_line(line: &[u64], k: usize) -> u64 {
    if k > line.len() {
        return 0;
    }
    let mut prod = line[..k].iter().filter(|&&x| x != 0).product();
    let mut zeros = line[..k].iter().filter(|&&x| x == 0).count();
    let mut res = if zeros == 0 { prod } else { 0 };
    for (&old, &new) in line.iter().zip(&line[k..]) {
        if old != 0 {
            prod /= old;
        } else {
            zeros -= 1;
        }
        if new != 0 {
            prod *= new;
        } else {
            zeros += 1;
        }
        if zeros == 0 {
            res = res.max(prod);
        }
    }
    println!("{line:?} {res}");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "70600674");
    }

    #[test]
    fn test_solve_line() {
        assert_eq!(solve_line(&vec![1, 2, 0, 4, 5], 0), 1);
        assert_eq!(solve_line(&vec![1, 2, 0, 4, 5], 1), 5);
        assert_eq!(solve_line(&vec![1, 2, 0, 4, 5], 2), 20);
        assert_eq!(solve_line(&vec![1, 2, 0, 4, 5], 3), 0);
        assert_eq!(solve_line(&vec![1, 2, 0, 4, 5], 4), 0);
        assert_eq!(solve_line(&vec![1, 2, 0, 4, 5], 5), 0);
        assert_eq!(solve_line(&vec![1, 2, 0, 4, 5], 6), 0);
        assert_eq!(solve_line(&vec![8, 9, 1, 4, 5, 6, 3], 2), 72);
        assert_eq!(solve_line(&vec![8, 9, 1, 4, 5, 6, 3], 3), 120);
    }

    #[test]
    fn test_solve_row() {
        assert_eq!(solve(&vec![vec![1, 6, 7], vec![2, 5, 6]], 2), 42);
        assert_eq!(solve(&vec![vec![1, 6, 7], vec![2, 5, 6]], 3), 60);
    }

    #[test]
    fn test_solve_col() {
        assert_eq!(solve(&vec![vec![1, 2], vec![6, 5], vec![7, 6]], 2), 42);
        assert_eq!(solve(&vec![vec![1, 2], vec![6, 5], vec![7, 6]], 3), 60);
    }

    #[test]
    fn test_solve_all() {
        assert_eq!(solve(&vec![vec![3, 3, 0], vec![0, 0, 0]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 3, 3], vec![0, 0, 0]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 0, 0], vec![3, 3, 0]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 0, 0], vec![0, 3, 3]], 2), 9);
        assert_eq!(solve(&vec![vec![3, 0, 0], vec![3, 0, 0]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 3, 0], vec![0, 3, 0]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 0, 3], vec![0, 0, 3]], 2), 9);
        assert_eq!(solve(&vec![vec![3, 0, 0], vec![0, 3, 0]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 3, 0], vec![0, 0, 3]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 3, 0], vec![3, 0, 0]], 2), 9);
        assert_eq!(solve(&vec![vec![0, 0, 3], vec![0, 3, 0]], 2), 9);
    }
}
