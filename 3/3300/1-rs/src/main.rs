// https://leetcode.com/problems/minimum-element-after-replacement-with-digit-sum/submissions/2016303016/?envType=daily-question&envId=2026-05-29

struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut best = i32::MAX;
        for i in 0..nums.len() {
            let mut num = nums[i];
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            if sum < best { best = sum }
        }
        best
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_element(vec![10, 12, 13, 14]);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_element(vec![1, 2, 3, 4]);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_element(vec![999, 19, 199]);
    let expected = 10;
    assert_eq!(actual, expected);
}
