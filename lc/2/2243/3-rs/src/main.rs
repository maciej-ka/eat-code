// https://leetcode.com/problems/calculate-digit-sum-of-a-string/submissions/2197579484/?envType=daily-question&envId=2026-11-16
struct Solution;

use std::mem::swap;
impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut round = String::with_capacity(s.len());
        let mut result = s;
        let k = k as usize;

        while result.len() > k {
            round.clear();
            result.as_bytes().chunks(k).for_each(|chunk| {
                let sum = chunk.iter().fold(0u32, |acc, x| acc + (x - b'0') as u32);
                round.push_str(&sum.to_string());
            });
            swap(&mut result, &mut round);
        }

        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::digit_sum(String::from("11111222223"), 3);
    let expected = String::from("135");
    assert_eq!(actual, expected);
}
