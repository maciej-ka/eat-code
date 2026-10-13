// https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/submissions/1815856898/

struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut last = 0;
        for v in target {
            if v > last { res += v - last }
            last = v;
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_number_operations(vec![1, 2, 3, 2, 1]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_number_operations(vec![3, 1, 1, 2]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_number_operations(vec![3, 1, 5, 4, 2]);
    let expected = 7;
    assert_eq!(actual, expected);
}
