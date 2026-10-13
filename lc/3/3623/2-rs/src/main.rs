// https://leetcode.com/problems/count-number-of-trapezoids-i/submissions/1844935867/

struct Solution;

use std::collections::HashMap;

const MOD: i64 = 1e9 as i64 + 7;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut counts = HashMap::<i32, i32>::new();
        for p in &points {
            counts.entry(p[1]).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut res = 0i64;
        let mut sum = 0i64;
        for val in counts.values() {
            let val = *val as i64;
            let val = val * (val - 1) % MOD >> 1;
            res = (res + val * sum % MOD) % MOD;
            sum = (sum + val) % MOD;
        }

        res as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::count_trapezoids(vec![vec![1,0],vec![2,0],vec![3,0],vec![2,2],vec![3,2]]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::count_trapezoids(vec![vec![0,0],vec![1,0],vec![0,1],vec![2,1]]);
    let expected = 1;
    assert_eq!(actual, expected);
}
