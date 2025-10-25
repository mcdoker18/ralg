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
