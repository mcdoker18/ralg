use core::panic;

// https://leetcode.com/problems/length-of-last-word/description/
pub fn length_of_last_word(line: String) -> i32 {
    let mut last_word_len = 0;
    let mut after_whitespace = true;

    for sym in line.chars() {
        if sym.is_whitespace() {
            after_whitespace = true;
        } else if after_whitespace {
            last_word_len = 1;
            after_whitespace = false;
        } else {
            last_word_len += 1;
        }
    }

    last_word_len
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    struct TestCase {
        word: &'static str,
        res: i32,
    }

    #[test]
    fn all() {
        let tests = HashMap::from([
            ("empty", TestCase { word: "", res: 0 }),
            (
                "one_word",
                TestCase {
                    word: "aaa",
                    res: 3,
                },
            ),
            (
                "one_word_after_space",
                TestCase {
                    word: " aaa",
                    res: 3,
                },
            ),
            (
                "one_word_after_space",
                TestCase {
                    word: " aaa",
                    res: 3,
                },
            ),
            (
                "one_word_after_multiple_spaces",
                TestCase {
                    word: "  aaa",
                    res: 3,
                },
            ),
            (
                "one_word_before_space",
                TestCase {
                    word: "aaa ",
                    res: 3,
                },
            ),
            (
                "one_word_before_multiple_spaces",
                TestCase {
                    word: "aaa  ",
                    res: 3,
                },
            ),
            (
                "two_words",
                TestCase {
                    word: " aaa bb",
                    res: 2,
                },
            ),
        ]);

        for (name, tc) in tests {
            assert_eq!(
                tc.res,
                length_of_last_word(tc.word.to_owned()),
                "name={name}"
            );
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum CounterName {
    A = 0,
    B = 1,
    C = 2,
}

impl CounterName {
    const COUNT: usize = 3;

    fn from_value(value: usize) -> Option<CounterName> {
        match value {
            0 => Some(CounterName::A),
            1 => Some(CounterName::B),
            2 => Some(CounterName::C),
            _ => None,
        }
    }
}

impl From<CounterName> for char {
    fn from(val: CounterName) -> char {
        match val {
            CounterName::A => 'a',
            CounterName::B => 'b',
            CounterName::C => 'c',
        }
    }
}

#[derive(Clone, Copy)]
struct CounterLastUsed {
    name: CounterName,
    used_times: usize,
}

struct Counters {
    values: [i32; CounterName::COUNT],
    last_used: Option<CounterLastUsed>,
    max_usage: usize,
}

impl Counters {
    fn new(max_usage: usize) -> Self {
        let values = [0; CounterName::COUNT];

        Counters {
            values,
            last_used: None,
            max_usage,
        }
    }

    fn set_value(&mut self, name: CounterName, value: i32) {
        self.values[name as usize] = value;
    }

    fn set_last_used(&mut self, name: CounterName) {
        self.values[name as usize] -= 1;
        if let Some(last_used) = &self.last_used
            && last_used.name == name
        {
            self.last_used = Some(CounterLastUsed {
                name,
                used_times: last_used.used_times + 1,
            });
        } else {
            self.last_used = Some(CounterLastUsed {
                name,
                used_times: 1,
            });
        }
    }
}

impl Iterator for Counters {
    type Item = CounterName;

    fn next(&mut self) -> Option<Self::Item> {
        let mut values = [(CounterName::A, 0); CounterName::COUNT];

        let mut j = 0;
        for (i, x) in self.values.iter().enumerate() {
            if *x == 0 {
                continue;
            }

            values[j] = (CounterName::from_value(i).unwrap(), *x);
            j += 1;
        }

        let values = &mut values[..j];
        if values.is_empty() {
            return None;
        }

        values.sort_by_key(|x| x.1);

        if let Some(last_used) = self.last_used {
            for v in values.iter().rev() {
                if v.0 == last_used.name && last_used.used_times >= self.max_usage {
                    continue;
                }

                self.set_last_used(v.0);
                return Some(v.0);
            }

            None
        } else {
            let last_used = values.last().unwrap().0;
            self.set_last_used(last_used);

            Some(last_used)
        }
    }
}

// https://leetcode.com/problems/longest-happy-string/description/
pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let max_length = a + b + c;
    assert!(max_length >= 0);

    let mut res = String::with_capacity(max_length as usize);

    let mut counters = Counters::new(2);

    counters.set_value(CounterName::A, a);
    counters.set_value(CounterName::B, b);
    counters.set_value(CounterName::C, c);

    for ch in counters {
        res.push(ch.into());
    }

    res
}

#[cfg(test)]
mod longest_diverse_string_tests {
    use super::*;

    struct TestCase {
        args: (i32, i32, i32),
        res: &'static str,
    }

    #[test]
    fn all() {
        let tests = [
            TestCase {
                args: (1, 1, 7),
                res: "ccbccacc",
            },
            TestCase {
                args: (7, 1, 0),
                res: "aabaa",
            },
            TestCase {
                args: (7, 1, 0),
                res: "aabaa",
            },
            TestCase {
                args: (0, 0, 0),
                res: "",
            },
            TestCase {
                args: (0, 1, 0),
                res: "b",
            },
            TestCase {
                args: (0, 2, 0),
                res: "bb",
            },
            TestCase {
                args: (0, 3, 0),
                res: "bb",
            },
        ];

        for tc in tests {
            assert_eq!(
                tc.res,
                longest_diverse_string(tc.args.0, tc.args.1, tc.args.2),
                "longest_diverse_string({:?})",
                tc.args
            );
        }
    }
}

// https://leetcode.com/problems/make-the-string-great/
pub fn make_string_great(s: String) -> String {
    let mut res: Vec<u8> = Vec::new();

    for ch in s.chars() {
        let last_ch = if let Some(last_ch) = res.last() {
            *last_ch as char
        } else {
            res.push(ch as u8);
            continue;
        };

        if (last_ch.is_uppercase() && last_ch.to_ascii_lowercase() == ch)
            || (last_ch.is_lowercase() && last_ch.to_ascii_uppercase() == ch)
        {
            res.pop();
        } else {
            res.push(ch as u8);
        }
    }

    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod make_string_great {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            ("", ""),
            ("a", "a"),
            ("A", "A"),
            ("Aa", ""),
            ("aA", ""),
            ("yAab", "yb"),
            ("AabB", ""),
            ("aaaaa", "aaaaa"),
            ("aaAaa", "aaa"),
            ("leEeetcode", "leetcode"),
            ("abBAcC", ""),
        ];

        for tc in tests {
            assert_eq!(tc.1, make_string_great(tc.0.to_owned()), "{}", tc.0)
        }
    }
}

// https://leetcode.com/problems/basic-calculator-ii/description/
// todo: replace with 2 stack later
pub fn basic_calculator2(s: String) -> i32 {
    let s = s.as_bytes();

    #[derive(Debug, Clone, Copy)]
    enum Sign {
        Plus,
        Minus,
        Multiplication,
        Division,
    }

    #[derive(Debug, Clone, Copy)]
    enum Expr {
        Number(i32),
        Sign(Sign),
        Empty,
    }

    let mut exprs = Vec::<Expr>::new();

    let mut i = 0;
    while i < s.len() {
        let curr_ch = s[i];

        if (curr_ch as char).is_whitespace() {
            i += 1;
            continue;
        }

        if (curr_ch as char).is_ascii_digit() {
            let mut digits = Vec::<u32>::new();
            digits.push((curr_ch as char).to_digit(10).unwrap());
            i += 1;

            while i < s.len() {
                let digit: u32 = match (s[i] as char).to_digit(10) {
                    Some(v) => v,
                    None => break,
                };

                digits.push(digit);
                i += 1;
            }

            let mut res: i32 = 0;

            for (i, digit) in digits.iter().rev().enumerate() {
                res += (*digit as i32) * 10_i32.pow(i as u32);
            }

            exprs.push(Expr::Number(res));

            continue;
        }

        let sign = match curr_ch as char {
            '*' => Sign::Multiplication,
            '/' => Sign::Division,
            '+' => Sign::Plus,
            '-' => Sign::Minus,
            other => panic!("unexpected symbol: {other}"),
        };
        i += 1;
        exprs.push(Expr::Sign(sign));
    }

    let get_next_num = |exprs: &Vec<Expr>, after_index: usize| -> (usize, i32) {
        for (i, expr) in exprs[after_index + 1..].iter().enumerate() {
            match expr {
                Expr::Number(v) => return (i + after_index + 1, *v),
                Expr::Empty => {}
                _ => panic!("unexpected expr: {expr:?}"),
            }
        }

        unreachable!();
    };

    let get_prev_num = |exprs: &Vec<Expr>, before_index: usize| -> (usize, i32) {
        for (i, expr) in exprs[0..before_index].iter().enumerate().rev() {
            match expr {
                Expr::Number(v) => return (i, *v),
                Expr::Empty => {}
                _ => panic!("unexpected expr: {expr:?}"),
            }
        }

        unreachable!();
    };

    let mut i = 0;
    while i < exprs.len() {
        if let Expr::Sign(sign) = exprs[i] {
            match sign {
                Sign::Multiplication => {
                    let (mut prev_index, prev_num) = get_prev_num(&exprs, i);
                    let (next_index, next_num) = get_next_num(&exprs, i);
                    exprs[prev_index] = Expr::Number(next_num * prev_num);

                    prev_index += 1;

                    while prev_index <= next_index {
                        exprs[prev_index] = Expr::Empty;
                        prev_index += 1;
                    }

                    i = next_index + 1;
                }
                Sign::Division => {
                    let (mut prev_index, prev_num) = get_prev_num(&exprs, i);
                    let (next_index, next_num) = get_next_num(&exprs, i);
                    exprs[prev_index] = Expr::Number(prev_num / next_num);

                    prev_index += 1;

                    while prev_index <= next_index {
                        exprs[prev_index] = Expr::Empty;
                        prev_index += 1;
                    }

                    i = next_index + 1;
                }
                _ => {
                    i += 1;
                }
            }
        } else {
            i += 1;
        }
    }

    let mut res = match exprs[0] {
        Expr::Number(v) => v,
        other => panic!("expected expr: {other:?}"),
    };

    let mut last_sign = Sign::Plus;

    for ex in exprs[1..].iter() {
        match ex {
            Expr::Number(v) => match last_sign {
                Sign::Plus => res += v,
                Sign::Minus => res -= v,
                Sign::Multiplication => res *= v,
                Sign::Division => res /= v,
            },
            Expr::Sign(sign) => last_sign = *sign,
            Expr::Empty => {}
        }
    }

    res
}

#[cfg(test)]
mod basic_calculator2_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            ("53", 53),
            ("1 + 2", 3),
            ("   1  +   2   ", 3),
            ("1 + 5 * 3", 16),
            ("7 + 12 / 3", 11),
        ];

        for tc in tests {
            assert_eq!(tc.1, basic_calculator2(tc.0.to_owned()), "{}", tc.0)
        }
    }
}

// https://leetcode.com/problems/word-pattern/description/
pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut pattern_cache = std::collections::HashMap::<char, &str>::with_capacity(pattern.len());
    let mut word_cache = std::collections::HashMap::<&str, char>::with_capacity(pattern.len());

    // todo: get rid of vector
    let s_splitted: Vec<&str> = s.split_whitespace().collect();
    if pattern.len() != s_splitted.len() {
        return false;
    }

    for (ch, word) in pattern.chars().zip(s_splitted) {
        match pattern_cache.entry(ch) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                if !(*occupied_entry.get()).eq(word) {
                    return false;
                }
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(word);
            }
        };

        match word_cache.entry(word) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                if !(*occupied_entry.get()).eq(&ch) {
                    return false;
                }
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(ch);
            }
        };
    }

    true
}

#[cfg(test)]
mod word_pattern_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            ("abba", "dog cat cat dog", true),
            ("abba", "dog cat cat fish", false),
            ("abba", "dog dog dog dog", false),
            ("aaa", "dog dog dog dog", false),
        ];

        for tc in tests {
            assert_eq!(
                tc.2,
                word_pattern(String::from(tc.0), String::from(tc.1)),
                "{} {}",
                tc.0,
                tc.1
            );
        }
    }
}

// https://leetcode.com/problems/determine-if-string-halves-are-alike/description
pub fn halves_are_alike(s: String) -> bool {
    fn count_vowels(bytes: &[u8]) -> usize {
        bytes
            .iter()
            .copied()
            .filter(|ch| {
                matches!(
                    *ch as char,
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
                )
            })
            .count()
    }

    let s = s.into_bytes();
    let s_len = s.len();

    count_vowels(&s[..s_len / 2]) == count_vowels(&s[s_len / 2..])
}

#[cfg(test)]
mod halves_are_alike_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            ("book", true),
            ("textbook", false),
            ("aa", true),
            ("ae", true),
            ("ab", false),
            ("bc", true),
        ];

        for tc in tests {
            assert_eq!(tc.1, halves_are_alike(String::from(tc.0)), "{}", tc.0);
        }
    }
}

// https://leetcode.com/problems/license-key-formatting/
#[allow(clippy::uninit_vec)]
pub fn license_key_formatting(s: String, k: i32) -> String {
    let k = k as usize;
    let bytes = s.into_bytes();

    let symbols_count = bytes.iter().copied().filter(|ch| *ch != b'-').count();
    let new_len = if symbols_count % k == 0 && symbols_count != 0 {
        symbols_count / k - 1
    } else {
        symbols_count / k
    } + symbols_count;

    if new_len == 0 {
        return String::from("");
    }

    let mut res = Vec::<u8>::with_capacity(new_len);
    unsafe {
        res.set_len(new_len);
    }

    let mut counter_k = 0;
    let mut res_w_i = res.len() - 1;

    for ch in bytes.iter().copied().rev() {
        if ch == b'-' {
            continue;
        }

        res[res_w_i] = (ch as char).to_ascii_uppercase() as u8;
        res_w_i = res_w_i.saturating_sub(1);

        counter_k += 1;

        if counter_k == k && res_w_i != 0 {
            res[res_w_i] = b'-';
            res_w_i = res_w_i.saturating_sub(1);

            counter_k = 0;
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}

#[cfg(test)]
mod license_key_formatting_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            ("5F3Z-2e-9-w", 4, "5F3Z-2E9W"),
            ("2-5g-3-J", 2, "2-5G-3J"),
            ("-", 1, ""),
        ];

        for tc in tests {
            assert_eq!(
                tc.2,
                license_key_formatting(String::from(tc.0), tc.1),
                "{} {}",
                tc.0,
                tc.1
            );
        }
    }
}
