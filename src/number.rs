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
