// https://leetcode.com/problems/complement-of-base-10-integer/submissions/1944824284/?envType=daily-question&envId=2026-03-11

struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut k = 1;
        while k < n { k = (k << 1) + 1 }
        k ^ n
    }
}

#[test]
fn test_1() {
    let actual = Solution::bitwise_complement(5);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::bitwise_complement(7);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::bitwise_complement(10);
    let expected = 5;
    assert_eq!(actual, expected);
}
