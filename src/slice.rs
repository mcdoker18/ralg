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

// https://leetcode.com/problems/find-peak-element/
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());

    if nums.len() == 1 {
        return 0;
    }

    let (mut i, mut j) = (0, nums.len() - 1);

    while i <= j {
        let mid_index = (j - i) / 2 + i;
        let mid_value = nums[mid_index];
        let after_mid_value = if mid_index != nums.len() - 1 {
            Some(nums[mid_index + 1])
        } else {
            None
        };
        let before_mid_value = if mid_index != 0 {
            Some(nums[mid_index - 1])
        } else {
            None
        };
        println!("i={i} j={j} mid_index={mid_index} vec={:?}", &nums);

        if after_mid_value.is_none_or(|v| mid_value > v)
            && before_mid_value.is_none_or(|v| mid_value > v)
        {
            return mid_index as i32;
        }

        if after_mid_value.is_some_and(|v| v > mid_value) {
            i = mid_index + 1;
            continue;
        }

        if before_mid_value.is_some_and(|v| v > mid_value) {
            j = mid_index;
            continue;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod find_peak_element_test {
    use super::*;

    #[test]
    fn all() {
        struct TestCase {
            nums: Vec<i32>,
            expected: i32,
        }

        let tests = [
            TestCase {
                nums: vec![1],
                expected: 0,
            },
            TestCase {
                nums: vec![1, 2],
                expected: 1,
            },
            TestCase {
                nums: vec![2, 1],
                expected: 0,
            },
            TestCase {
                nums: vec![1, 2, 3, 1],
                expected: 2,
            },
            TestCase {
                nums: vec![1, 2, 1, 3, 5, 6, 4],
                expected: 5,
            },
        ];

        for tc in tests {
            let desc = format!("vec={:?}", tc.nums);

            assert_eq!(tc.expected, find_peak_element(tc.nums), "{desc}");
        }
    }
}
