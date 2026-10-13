// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/submissions/2156323888/?envType=daily-question&envId=2026-09-28
struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut result = 0;
        let mut current = 0;
        for char in s.chars() {
            if char == '(' { current += 1; }
            if char == ')' { current -= 1; }
            result = result.max(current);
        }
        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_depth(String::from("(1+(2*3)+((8)/4))+1"));
    let expected = 3;
    assert_eq!(actual, expected);
}
