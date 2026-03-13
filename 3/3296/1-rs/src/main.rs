// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/submissions/1947421727/?envType=daily-question&envId=2026-03-13

struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

// 0: next total, 1: base, 2: times
type WorkerTime = (i64, i64, i64);

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut heap = BinaryHeap::<Reverse<WorkerTime>>::new();
        for t in worker_times { heap.push(Reverse((t as i64, t as i64, 1))); }

        for i in 0..mountain_height {
            let mut t = heap.pop().unwrap();
            ans = ans.max(t.0.0);
            t.0.2 += 1;
            t.0.0 += t.0.1 * t.0.2;
            heap.push(t);
        }

        ans
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_number_of_seconds(4, vec![2,1,1]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_number_of_seconds(10, vec![3,2,2,4]);
    let expected = 12;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_number_of_seconds(5, vec![1]);
    let expected = 15;
    assert_eq!(actual, expected);
}
