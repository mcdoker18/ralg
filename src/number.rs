// https://leetcode.com/problems/maximum-69-number/
pub fn maximum69_number(num: i32) -> i32 {
    let mut reminder: u32 = num as u32;

    for x in (0..4_u32).rev() {
        let base = u32::pow(10, x);
        let digit = reminder / base;
        reminder %= base;

        if digit == 6 {
            return num + (3 * base) as i32;
        }
    }

    num
}

#[cfg(test)]
mod maximum69_number_tests {
    use super::*;

    #[test]
    fn all() {
        let tests = [(6, 9), (9, 9), (6666, 9666), (9666, 9966), (9699, 9999)];

        for tc in tests {
            assert_eq!(tc.1, maximum69_number(tc.0), "arg={}", tc.0);
        }
    }
}

// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
pub fn replace_elements_with_greatest(mut arr: Vec<i32>) -> Vec<i32> {
    assert!(!arr.is_empty());

    let mut curr_max = *arr.last().unwrap();
    *(arr.last_mut().unwrap()) = -1;

    for elem in arr.iter_mut().rev().skip(1) {
        let elem_value = *elem;
        *elem = curr_max;

        if elem_value > curr_max {
            curr_max = elem_value;
        }
    }

    arr
}

#[cfg(test)]
mod replace_elements_with_greatest_tests {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (vec![1], vec![-1]),
            (vec![2], vec![-1]),
            (vec![3, 4], vec![4, -1]),
            (vec![5, 4], vec![4, -1]),
            (vec![17, 18, 5, 4, 6, 1], vec![18, 6, 6, 6, 1, -1]),
            (vec![5, 5, 5, 3, 2, 4], vec![5, 5, 4, 4, 4, -1]),
        ];

        for tc in tests {
            let desc = format!("{:?}", &tc.0);
            assert_eq!(tc.1, replace_elements_with_greatest(tc.0), "{desc}");
        }
    }
}
