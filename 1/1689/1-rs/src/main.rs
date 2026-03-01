// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/submissions/1934832668/?envType=daily-question&envId=2026-03-01

struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        (n.bytes().max().unwrap() - b'0') as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_partitions(String::from("32"));
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_partitions(String::from("82734"));
    let expected = 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_partitions(String::from("27346209830709182346"));
    let expected = 9;
    assert_eq!(actual, expected);
}
