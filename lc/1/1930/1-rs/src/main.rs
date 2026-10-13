// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/submissions/1836186467/

struct Solution;

const LEN: usize = (b'z' - b'a') as usize + 1;
fn ind(a: u8) -> usize { (a - b'a') as usize }

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut first = [1000000; LEN];
        let mut last = [0; LEN];
        let mut walk = vec![Vec::with_capacity(s.len() + 1).clone(); LEN];
        for i in 0..walk.len() { walk[i].push(0) }

        for (i, c) in s.bytes().enumerate() {
            for k in 0..LEN {
                let val = walk[k][i];
                walk[k].push(val);
            }
            let index = ind(c);
            walk[index][i + 1] += 1;
            first[index] = first[index].min(i);
            last[index] = last[index].max(i);
        }

        let mut res = 0;
        for i in 0..LEN {
            let lo = first[i];
            let hi = last[i];
            if hi == 0 || lo == hi { continue }
            for k in 0..LEN {
                if walk[k][hi] - walk[k][lo + 1] > 0 {
                    res += 1
                }
            }
        }

        res
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
