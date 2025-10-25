use std::{cell::RefCell, collections::HashMap, rc::Rc};

// https://leetcode.com/problems/implement-trie-prefix-tree/description/
#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    nodes: HashMap<char, TrieNode>,
    is_word: bool,
}

impl Trie {
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.nodes.entry(ch).or_default();
        }

        node.is_word = true;
    }

    pub fn search(&mut self, word: String) -> bool {
        let mut node = &self.root;

        for ch in word.chars() {
            node = match node.nodes.get(&ch) {
                Some(value) => value,
                None => return false,
            };
        }

        node.is_word
    }

    pub fn start_with(&mut self, prefix: String) -> bool {
        if prefix.is_empty() {
            return self.root.is_word || !self.root.nodes.is_empty();
        }

        let mut node = &self.root;

        for ch in prefix.chars() {
            node = match node.nodes.get(&ch) {
                Some(value) => value,
                None => return false,
            };
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut t = Trie::default();

        assert!(!t.search("".to_owned()));
        assert!(!t.start_with("".to_owned()));

        t.insert("".to_owned());

        assert!(t.search("".to_owned()));
        assert!(t.start_with("".to_owned()));

        for word in ["a", "aa"] {
            assert!(!t.search(word.to_owned()), "word {word}");
        }

        for word in ["aaaa", "b", "bb"] {
            assert!(!t.start_with(word.to_owned()), "word {word}");
        }
    }

    #[test]
    fn one_word() {
        let mut t = Trie::default();

        t.insert("aaa".to_owned());

        assert!(t.search("aaa".to_owned()));

        for word in ["", "a", "aa", "aaaa", "b"] {
            assert!(!t.search(word.to_owned()), "word {word}");
        }

        for word in ["", "a", "aa", "aaa"] {
            assert!(t.start_with(word.to_owned()), "word {word}");
        }

        for word in ["aaaa", "b", "bb"] {
            assert!(!t.start_with(word.to_owned()), "word {word}");
        }
    }

    #[test]
    fn multiple_same_word() {
        let mut t = Trie::default();

        t.insert("aaa".to_owned());
        t.insert("a".to_owned());

        for word in ["a", "aaa"] {
            assert!(t.search(word.to_owned()), "word {word}");
        }

        for word in ["", "aa", "aaaa", "b"] {
            assert!(!t.search(word.to_owned()), "word {word}");
        }

        for word in ["", "a", "aa", "aaa"] {
            assert!(t.start_with(word.to_owned()), "word {word}");
        }

        for word in ["aaaa", "b", "bb"] {
            assert!(!t.start_with(word.to_owned()), "word {word}");
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root = match root {
        Some(v) => v,
        None => return 0,
    };

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![root];
    let mut counter = 0;

    while let Some(node) = stack.pop() {
        counter += 1;
        let item = node.borrow();

        if let Some(value) = &item.left {
            stack.push(Rc::clone(value));
        }

        if let Some(value) = &item.right {
            stack.push(Rc::clone(value));
        }
    }

    counter
}
