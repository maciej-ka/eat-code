// https://leetcode.com/problems/left-and-right-sum-differences/submissions/2024228362/?envType=daily-question&envId=2026-06-06

struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::<i32>::with_capacity(nums.len());
        let mut right: i32 = nums.iter().sum();
        let mut left = 0;
        for n in nums {
            right -= n;
            let diff = right - left;
            res.push(diff.abs());
            left += n;
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::left_right_difference(vec![10, 4, 8, 3]);
    let expected = [15, 1, 11, 22];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::left_right_difference(vec![1]);
    let expected = [0];
    assert_eq!(actual, expected);
}
