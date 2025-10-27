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
