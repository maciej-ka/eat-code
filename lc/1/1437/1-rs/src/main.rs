// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/submissions/1832143028/

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut last = None;
        for hi in 0..nums.len() {
            if nums[hi] == 1 {
                if let Some(lo) = last {
                    if hi - lo <= k { return false }
                }
                last = Some(hi);
            }
        }
        true
    }
}

#[test]
fn test_1() {
    let actual = Solution::k_length_apart(vec![1,0,0,0,1,0,0,1], 2);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::k_length_apart(vec![1,0,0,1,0,1], 2);
    let expected = false;
    assert_eq!(actual, expected);
}
