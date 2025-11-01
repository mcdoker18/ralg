// https://leetcode.com/problems/maximum-product-subarray/
pub fn max_product(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());

    let mut current: i32 = 1;
    let mut current_after_negative: i32 = 1;
    let mut was_negative = false;
    let mut max = i32::MIN;

    for x in nums {
        if x == 0 {
            max = std::cmp::max(max, x);
            current = 1;
            current_after_negative = 1;
            was_negative = false;

            continue;
        }

        if was_negative {
            current *= x;
            current_after_negative *= x;

            max = std::cmp::max(max, std::cmp::max(current, current_after_negative));
        } else {
            if x < 0 {
                was_negative = true;
                current_after_negative = 1;
            }

            current *= x;
            max = std::cmp::max(max, current);
        }
    }

    max
}

#[cfg(test)]
mod max_product_test {
    use crate::slice::max_product;

    #[test]
    fn all() {
        struct TestCase {
            nums: Vec<i32>,
            expected: i32,
        }

        let tests = [
            TestCase {
                nums: vec![5],
                expected: 5,
            },
            TestCase {
                nums: vec![5, 1, 2],
                expected: 10,
            },
            TestCase {
                nums: vec![2, 3, -2, 4],
                expected: 6,
            },
            TestCase {
                nums: vec![-2, 0, -1],
                expected: 0,
            },
            TestCase {
                nums: vec![0, 2],
                expected: 2,
            },
            TestCase {
                nums: vec![2, -5, -2, -4, 3],
                expected: 24,
            },
        ];

        for tc in tests {
            let desc = format!("nums={:?}", &tc.nums);

            assert_eq!(tc.expected, max_product(tc.nums), "{desc}");
        }
    }
}
