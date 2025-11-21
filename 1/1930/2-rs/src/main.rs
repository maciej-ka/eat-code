// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/submissions/1836197104/

struct Solution;

use std::collections::HashSet;

const LEN: usize = (b'z' - b'a') as usize + 1;
fn ind(a: u8) -> usize { (a - b'a') as usize }

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut first = [1000000; LEN];
        let mut last = [0; LEN];

        for (i, c) in s.bytes().enumerate() {
            let index = ind(c);
            first[index] = first[index].min(i);
            last[index] = last[index].max(i);
        }

        let mut res = 0;
        for i in 0..LEN {
            let lo = first[i];
            let hi = last[i];
            if hi == 0 || lo == hi { continue }

            let mut set = HashSet::new();
            for c in s[lo+1..hi].bytes() { set.insert(c); }
            res += set.len();
        }

        res as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::count_palindromic_subsequence(String::from("aabca"));
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::count_palindromic_subsequence(String::from("adc"));
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::count_palindromic_subsequence(String::from("bbcbaba"));
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::count_palindromic_subsequence(String::from("uuuuuc"));
    let expected = 1;
    assert_eq!(actual, expected);
}
