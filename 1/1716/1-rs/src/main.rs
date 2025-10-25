// https://leetcode.com/problems/calculate-money-in-leetcode-bank/submissions/1811078035/

struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut result: i32 = 0;
        let mut week = 0;
        for i in 0..n {
            let r = i % 7;
            if r == 0 { week += 1 }
            result += r + week;
        }
        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::total_money(4);
    let expected = 10;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::total_money(10);
    let expected = 37;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::total_money(20);
    let expected = 96;
    assert_eq!(actual, expected);
}
