// https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification/submissions/2202219545/?envType=daily-question&envId=2026-11-20

struct Solution;

impl Solution {
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let len = nums.len();
        let mut dp = vec![0; nums[len - 1] as usize + 3];
        let mut res = 1i32;

        for i in (0..nums.len()).rev() {
            let num = nums[i] as usize;
            dp[num] = dp[num + 1] + 1;
            dp[num + 1] = dp[num + 2] + 1;
            res = res.max(dp[num]).max(dp[num + 1]);
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_selected_elements(vec![2,1,5,1,1]);
    let expected = 3;
    assert_eq!(actual, expected);
}
