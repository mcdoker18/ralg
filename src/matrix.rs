use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::VecDeque,
};

// https://leetcode.com/problems/search-a-2d-matrix-ii/description/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let (row_len, col_len) = (matrix.len(), matrix[0].len());
    let (mut i, mut j) = (0, col_len - 1);

    loop {
        let value = matrix[i][j];

        match target.cmp(&value) {
            Less => {
                if j == 0 {
                    return false;
                }

                j -= 1;
            }
            Equal => return true,
            Greater => {
                if i == row_len - 1 {
                    return false;
                }

                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod search_matrix_test {
    use crate::matrix::search_matrix;

    #[test]
    fn all() {
        struct TestCase {
            matrix: Vec<Vec<i32>>,
            target: i32,
            expected: bool,
        }

        let test_cases = [
            TestCase {
                matrix: vec![],
                target: 5,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![]],
                target: 5,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![1]],
                target: 3,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![1]],
                target: 1,
                expected: true,
            },
            TestCase {
                matrix: vec![vec![1, 5, 7, 9]],
                target: 5,
                expected: true,
            },
            TestCase {
                matrix: vec![vec![1, 5, 7, 9]],
                target: -5,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![1], vec![5], vec![7], vec![9]],
                target: 5,
                expected: true,
            },
            TestCase {
                matrix: vec![vec![1], vec![5], vec![7], vec![9]],
                target: 11,
                expected: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, search_matrix(tc.matrix, tc.target))
        }
    }
}

// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/
pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let mut maze = maze;

    if maze.is_empty() || maze[0].is_empty() {
        return -1;
    }

    const EMPTY_CELL: char = '.';
    const VISISTED_CELL: char = '*';

    let (maze_col_len, maze_row_len) = (maze.len(), maze[0].len());
    let entrance = (entrance[0] as usize, entrance[1] as usize);

    assert!(entrance.0 < maze_col_len);
    assert!(entrance.1 < maze_row_len);

    assert!(maze[entrance.0][entrance.1] == EMPTY_CELL);
    maze[entrance.0][entrance.1] = VISISTED_CELL;

    let mut steps: VecDeque<(usize, usize)> = VecDeque::new();

    if entrance.0 != maze_col_len - 1 && maze[entrance.0 + 1][entrance.1] == EMPTY_CELL {
        maze[entrance.0 + 1][entrance.1] = VISISTED_CELL;
        steps.push_back((entrance.0 + 1, entrance.1));
    }

    if entrance.0 != 0 && maze[entrance.0 - 1][entrance.1] == EMPTY_CELL {
        maze[entrance.0 - 1][entrance.1] = VISISTED_CELL;
        steps.push_back((entrance.0 - 1, entrance.1));
    }

    if entrance.1 != maze_row_len - 1 && maze[entrance.0][entrance.1 + 1] == EMPTY_CELL {
        maze[entrance.0][entrance.1 + 1] = VISISTED_CELL;
        steps.push_back((entrance.0, entrance.1 + 1));
    }

    if entrance.1 != 0 && maze[entrance.0][entrance.1 - 1] == EMPTY_CELL {
        maze[entrance.0][entrance.1 - 1] = VISISTED_CELL;
        steps.push_back((entrance.0, entrance.1 - 1));
    }

    if steps.is_empty() {
        return -1;
    }

    let mut res = 1;

    while !steps.is_empty() {
        let number_of_steps = steps.len();

        for _ in 0..number_of_steps {
            let step = steps.pop_front().unwrap();

            if step.0 == 0
                || step.0 == maze_col_len - 1
                || step.1 == 0
                || step.1 == maze_row_len - 1
            {
                return res;
            }

            if maze[step.0 + 1][step.1] == EMPTY_CELL {
                maze[step.0 + 1][step.1] = VISISTED_CELL;
                steps.push_back((step.0 + 1, step.1));
            }

            if maze[step.0 - 1][step.1] == EMPTY_CELL {
                maze[step.0 - 1][step.1] = VISISTED_CELL;
                steps.push_back((step.0 - 1, step.1));
            }

            if maze[step.0][step.1 + 1] == EMPTY_CELL {
                maze[step.0][step.1 + 1] = VISISTED_CELL;
                steps.push_back((step.0, step.1 + 1));
            }

            if maze[step.0][step.1 - 1] == EMPTY_CELL {
                maze[step.0][step.1 - 1] = VISISTED_CELL;
                steps.push_back((step.0, step.1 - 1));
            }
        }

        res += 1;

        if steps.is_empty() {
            return -1;
        }
    }

    res
}

#[cfg(test)]
mod nearest_exit_test {
    use crate::matrix::nearest_exit;

    #[test]
    fn all() {
        struct TestCase {
            matrix: Vec<Vec<char>>,
            entrance: Vec<i32>,
            expected: i32,
        }

        let test_cases = [
            TestCase {
                matrix: vec![vec![]],
                entrance: vec![],
                expected: -1,
            },
            TestCase {
                matrix: vec![
                    vec!['+', '+', '+'],
                    vec!['+', '.', '+'],
                    vec!['+', '+', '+'],
                ],
                entrance: vec![1, 1],
                expected: -1,
            },
            TestCase {
                matrix: vec![
                    vec!['.', '+', '.'],
                    vec!['+', '.', '+'],
                    vec!['.', '+', '.'],
                ],
                entrance: vec![1, 1],
                expected: -1,
            },
            TestCase {
                matrix: vec![
                    vec!['.', '.', '.'],
                    vec!['+', '.', '+'],
                    vec!['.', '+', '.'],
                ],
                entrance: vec![1, 1],
                expected: 1,
            },
            TestCase {
                matrix: vec![
                    vec!['.', '.', '.'],
                    vec!['+', '.', '+'],
                    vec!['.', '+', '.'],
                ],
                entrance: vec![0, 1],
                expected: 1,
            },
            TestCase {
                matrix: vec![
                    vec!['+', '.', '+'],
                    vec!['+', '.', '+'],
                    vec!['.', '+', '.'],
                ],
                entrance: vec![0, 1],
                expected: -1,
            },
            TestCase {
                matrix: vec![
                    vec!['+', '+', '+', '+', '+'],
                    vec!['+', '.', '.', '.', '+'],
                    vec!['+', '.', '.', '.', '+'],
                    vec!['+', '.', '.', '.', '+'],
                    vec!['+', '+', '+', '+', '+'],
                ],
                entrance: vec![2, 2],
                expected: -1,
            },
            TestCase {
                matrix: vec![
                    vec!['+', '+', '+', '+', '+'],
                    vec!['+', '.', '.', '.', '+'],
                    vec!['+', '.', '+', '.', '+'],
                    vec!['+', '.', '.', '.', '+'],
                    vec!['+', '+', '+', '+', '+'],
                ],
                entrance: vec![2, 3],
                expected: -1,
            },
            TestCase {
                matrix: vec![
                    vec!['+', '+', '+', '+', '+'],
                    vec!['+', '.', '.', '.', '+'],
                    vec!['+', '.', '.', '.', '+'],
                    vec!['.', '.', '.', '.', '+'],
                    vec!['+', '+', '+', '+', '+'],
                ],
                entrance: vec![2, 2],
                expected: 3,
            },
        ];

        for tc in test_cases {
            let test_name = format!(
                "matrix={:?} entrance={:?} expected={}",
                tc.matrix, tc.entrance, tc.expected
            );

            assert_eq!(
                tc.expected,
                nearest_exit(tc.matrix, tc.entrance),
                "{test_name}"
            );
        }
    }
}

// https://leetcode.com/problems/minimum-path-sum/
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    assert!(!grid.is_empty());
    assert!(!grid[0].is_empty());

    let n = grid.len();
    let m = grid[0].len();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];
    dp[0][0] = grid[0][0];

    for i in 1..n {
        dp[i][0] = grid[i][0] + dp[i - 1][0];
    }

    for j in 1..m {
        dp[0][j] = grid[0][j] + dp[0][j - 1];
    }

    for i in 1..n {
        for j in 1..m {
            dp[i][j] = grid[i][j] + std::cmp::min(dp[i - 1][j], dp[i][j - 1]);
        }
    }

    dp[n - 1][m - 1]
}

#[cfg(test)]
mod min_path_sum_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (vec![vec![1, 2, 3], vec![4, 5, 6]], 12),
            (vec![vec![3]], 3),
            (vec![vec![3, 2]], 5),
            (vec![vec![3], vec![2]], 5),
            (vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]], 7),
        ];

        for tc in tests {
            let desc = format!("matrix={:?}", tc.0);

            assert_eq!(tc.1, min_path_sum(tc.0), "{desc}");
        }
    }
}
