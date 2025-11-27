// https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/submissions/1841171564/

struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut sum = 0i64;
        for i in 0..k { sum += nums[i] as i64 }

        let mut best = sum;
        let mut memo = vec![0i64; k];
        memo[0] = sum;

        for i in k..nums.len() {
            let j = (i + 1) % k;
            sum += (nums[i] - nums[i - k]) as i64;
            memo[j] = sum + memo[j].max(0);
            best = best.max(memo[j]);
        }

        best
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_subarray_sum(vec![1, 2], 1);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4);
    let expected = -10;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::max_subarray_sum(vec![2, 2, -1, -2, 1, 1], 2);
    let expected = 4;
    assert_eq!(actual, expected);
}
