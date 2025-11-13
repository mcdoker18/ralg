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

// https://leetcode.com/problems/integer-break/description/
pub fn integer_break(n: i32) -> i32 {
    let mut cache = vec![0; std::cmp::max(6, (n + 1) as usize)];
    cache[2] = 1;
    cache[3] = 2;
    cache[4] = 4;
    cache[5] = 6;

    fn find_max(n: i32, cache: &mut [i32]) -> i32 {
        let value = cache[n as usize];
        if value != 0 {
            return value;
        }

        let mut max = 0;

        for num_elems in 2..=n / 2 {
            for inc_value in 2..=n / 2 {
                'fill: for fill_inc_value in 0..num_elems {
                    let mut remaining = n;
                    let mut res = 1;

                    for _ in 0..=fill_inc_value {
                        res *= inc_value;
                        remaining -= inc_value;

                        if remaining < 2 {
                            break 'fill;
                        }
                    }

                    for _ in (fill_inc_value + 1)..(num_elems - 1) {
                        res *= 2;
                        remaining -= 2;
                        if remaining < 2 {
                            break 'fill;
                        }
                    }

                    res *= remaining;

                    max = std::cmp::max(max, res);
                }
            }
        }

        max
    }

    find_max(n, &mut cache)
}

#[cfg(test)]
mod integer_break_tests {
    use super::*;

    #[test]
    fn all() {
        let tests = [(5, 6), (10, 36), (6, 9), (12, 81)];

        for tc in tests {
            assert_eq!(tc.1, integer_break(tc.0), "num={}", tc.0);
        }
    }
}

// https://leetcode.com/problems/power-of-two/
pub fn is_power_of_two(n: i32) -> bool {
    (n > 0) && (n & (n - 1) == 0)
}

#[cfg(test)]
mod is_power_of_two_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (-1, false),
            (0, false),
            (1, true),
            (2, true),
            (3, false),
            (4, true),
            (5, false),
            (6, false),
            (8, true),
        ];

        for tc in tests {
            assert_eq!(tc.1, is_power_of_two(tc.0), "{}", tc.0);
        }
    }
}

// https://leetcode.com/problems/ugly-number/
pub fn is_ugly(n: i32) -> bool {
    match n {
        ..=0 => return false,
        1..=5 => return true,
        _ => (),
    }

    const PRIMES: [i32; 3] = [5, 3, 2];

    for prime in PRIMES {
        if n % prime == 0 && is_ugly(n / prime) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod is_ugly_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (-20, false),
            (-2, false),
            (-1, false),
            (0, false),
            (1, true),
            (2, true),
            (3, true),
            (5, true),
            (4, true),
            (6, true),
            (7, false),
            (8, true),
            (9, true),
            (10, true),
            (11, false),
            (22, false),
            (24, true),
        ];

        for tc in tests {
            assert_eq!(tc.1, is_ugly(tc.0), "{}", tc.0);
        }
    }
}
