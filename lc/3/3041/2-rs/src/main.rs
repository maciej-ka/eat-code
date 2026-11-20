// https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification/submissions/?envType=daily-question&envId=2026-11-20

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut counts = HashMap::<i32,i32>::new();

        for num in nums {
            counts.entry(num).and_modify(|x| *x += 1).or_insert(1);
            max = max.max(num);
        }

        let mut best = 1;
        let mut current = 0;
        let mut double = 0;
        let mut left = 0;
        let mut next_left = 0;

        for num in 0..=max+1 {
            let count = counts.get(&num).unwrap_or(&0);

            // no gap
            if *count > 0 {
                current += 1;
                next_left += 1;
                if *count > 1 { double = 1 }
                continue;
            }

            // double gap
            if !counts.contains_key(&(num + 1)) {
                best = best.max(left + current + double);
                current = 0;
                left = 0;
                next_left = 0;
                double = 0;
                continue;
            }

            // single gap, double present
            if double > 0 {
                current += 1;
                next_left = 0;
                double = 0;
                continue;
            }

            // single gap, no double
            best = best.max(left + current);
            left = next_left;
            next_left = 0;
            current = 0;
        }

        best
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_selected_elements(vec![2,1,5,1,1]);
    let expected = 3;
    assert_eq!(actual, expected);
}
