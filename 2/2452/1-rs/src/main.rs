// https://leetcode.com/problems/words-within-two-edits-of-dictionary/submissions/1985579112/?envType=daily-question&envId=2026-04-22

struct Solution;

use std::iter::zip;

impl Solution {
    fn is_within(a: &str, b: &str) -> bool {
        let mut edits = 0;
        for (char_a, char_b) in zip(a.chars(), b.chars()) {
            if char_a != char_b {
                if edits >= 2 { return false }
                edits += 1;
            }
        }
        true
    }

    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries
            .into_iter()
            .filter(|q| dictionary
                .iter()
                .any(|d| Self::is_within(q, d)))
            .collect()
    }
}

#[test]
fn test_1() {
    let actual = Solution::two_edit_words(
        vec![String::from("word"), String::from("note"), String::from("ants"), String::from("wood")],
        vec![String::from("wood"), String::from("joke"), String::from("moat")]
    );
    let expected = vec![String::from("word"), String::from("note"), String::from("wood")];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::two_edit_words(vec![String::from("yes")], vec![String::from("not")]);
    let expected = Vec::<String>::new();
    assert_eq!(actual, expected);
}
