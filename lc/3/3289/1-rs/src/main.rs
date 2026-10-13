// https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/submissions/1816696990/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        let mut res = Vec::new();
        for n in nums {
            if set.contains(&n) {
                res.push(n);
            } else {
                set.insert(n);
            }
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::get_sneaky_numbers(vec![0, 1, 1, 0]);
    let expected = vec![1, 0];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]);
    let expected = vec![3, 2];
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]);
    let expected = vec![4, 5];
    assert_eq!(actual, expected);
}
