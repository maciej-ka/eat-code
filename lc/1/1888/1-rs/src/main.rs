// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/submissions/1941091902/?envType=daily-question&envId=2026-03-07

struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let mut c0 = 0;
        let mut c1 = 0;
        let mut s0 = Vec::<i32>::with_capacity(n);
        let mut s1 = Vec::<i32>::with_capacity(n);

        for (i, c) in s.bytes().enumerate() {
            if (i % 2 == 0) == (c == b'0') {
                c0 += 1;
            } else {
                c1 += 1;
            }
            s0.push(c0);
            s1.push(c1);
        }

        let s0last = s0[n - 1];
        let s1last = s1[n - 1];
        let mut best = s0last.min(s1last);
        if n % 2 == 0 { return best }

        for i in 0..n {
            best = best.min(s0[i] + s1last - s1[i]);
            best = best.min(s1[i] + s0last - s0[i]);
        }
        best
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_flips(String::from("111000"));
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_flips(String::from("010"));
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_flips(String::from("1110"));
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::min_flips(String::from("101001010"));
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::min_flips(String::from("101100101"));
    let expected = 2;
    assert_eq!(actual, expected);
}
