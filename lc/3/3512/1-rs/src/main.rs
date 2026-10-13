// https://leetcode.com/problems/minimum-operations-to-make-array-sum-divisible-by-k/submissions/1842446683/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_operations(vec![3, 9, 7], 5);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_operations(vec![4, 1, 3], 4);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_operations(vec![3, 2], 6);
    let expected = 5;
    assert_eq!(actual, expected);
}
