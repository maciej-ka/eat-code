// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/submissions/1836779494/

struct Solution;

const LCOUNT: usize = (b'z' - b'a') as usize + 1;
fn ind(a: u8) -> usize { (a - b'a') as usize }

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut first = [1000000; LCOUNT];
        let mut last = [0; LCOUNT];

        for (i, c) in s.bytes().enumerate() {
            let index = ind(c);
            first[index] = first[index].min(i);
            last[index] = i;
        }

        let mut res = 0;
        let mut arr = [0; LCOUNT];
        for i in 0..LCOUNT {
            let lo = first[i];
            let hi = last[i];
            if hi == 0 || lo == hi { continue }
            arr.fill(0);
            for c in s[lo+1..hi].bytes() { arr[ind(c)] = 1; }
            res += arr.iter().sum::<i32>();
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
