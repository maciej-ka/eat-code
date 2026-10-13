// https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/submissions/1828558387/

struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut res = 0;
        let mut ones = 0;
        let mut last = '0';

        for c in s.chars() {
            if c == '0' && last == '1' { res += ones }
            if c == '1' { ones += 1 }
            last = c
        }

        res as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_operations(String::from("1001101"));
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_operations(String::from("00111"));
    let expected = 0;
    assert_eq!(actual, expected);
}
