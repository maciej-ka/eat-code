// https://leetcode.com/problems/minimum-score-by-changing-two-elements/submissions/2199603412/?envType=daily-question&envId=2026-11-19

struct Solution;

impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 3 { return 0; }

        let mut nums = nums;
        nums.sort_unstable();

        let mut res = i32::MAX;
        for i in 0..3 {
            for k in (len - 3 + i)..len {
                let diff = nums[i] - nums[k];
                res = res.min(diff.abs());
            }
        }

        res
    }
}



#[test]
fn test_1() {
    let actual = Solution::minimize_sum(vec![1, 4, 7, 8, 5]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::minimize_sum(vec![1, 4, 3]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::minimize_sum(vec![21,13,21,72,35,52,74]);
    let expected = 39;
    assert_eq!(actual, expected);
}
