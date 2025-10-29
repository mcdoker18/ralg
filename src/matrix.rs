use std::cmp::Ordering::{Equal, Greater, Less};

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
