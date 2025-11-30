// https://leetcode.com/problems/make-sum-divisible-by-p/submissions/1843689257/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut total = 0;
        for i in 0..nums.len() {
            total = (total + nums[i]) % p
        }

        // all elements divide
        if total == 0 { return 0 }

        let mut best = nums.len() as i32;
        let mut modulo = 0;
        let mut pos = HashMap::<i32, i32>::new();
        pos.insert(0, -1);

        for i in 0..nums.len() {
            modulo = (modulo + nums[i]) % p;
            pos.insert(modulo, i as i32);
            let need = (p + modulo - total) % p;
            if let Some(k) = pos.get(&need) {
                best = best.min(i as i32 - k);
            }
        }

        if best == nums.len() as i32 { -1 } else { best }
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_subarray(vec![3, 1, 4, 2], 6);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_subarray(vec![6, 3, 5, 2], 9);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_subarray(vec![1, 2, 3], 3);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::min_subarray(vec![8, 32, 31, 18, 34, 20, 21, 13, 1, 27, 23, 22, 11, 15, 30, 4, 2], 148);
    let expected = 7;
    assert_eq!(actual, expected);
}
