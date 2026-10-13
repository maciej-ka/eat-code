// https://leetcode.com/problems/maximum-total-subarray-value-i/submissions/2027166913/?envType=daily-question&envId=2026-06-09

struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let mut max = nums[0];
        let mut min = nums[0];
        for n in nums {
            max = max.max(n);
            min = min.min(n);
        }
        (max - min) as i64 * k as i64
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_total_value(vec![1, 3, 2], 2);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_total_value(vec![4, 2, 5, 1], 3);
    let expected = 12;
    assert_eq!(actual, expected);
}
