// https://leetcode.com/problems/binary-number-with-alternating-bits/submissions/1923761577/
struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut last = n & 1;
        while true {
            n >>= 1;
            if n == 0 { break }
            if n & 1 == last { return false }
            last = n & 1;
        }
        true
    }
}

#[test]
fn test_1() {
    let actual = Solution::has_alternating_bits(5);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::has_alternating_bits(7);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::has_alternating_bits(11);
    let expected = false;
    assert_eq!(actual, expected);
}
