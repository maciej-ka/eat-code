// https://leetcode.com/problems/smallest-number-with-all-set-bits/submissions/1814858507/

struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        if n == 1 { return 1 }
        let mut res = 1;
        while res <= n { res = res << 1 }
        res - 1
    }
}

#[test]
fn test_1() {
    let actual = Solution::smallest_number(5);
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::smallest_number(10);
    let expected = 15;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::smallest_number(3);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::smallest_number(1);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::smallest_number(4);
    let expected = 7;
    assert_eq!(actual, expected);
}
