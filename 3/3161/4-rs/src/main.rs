// https://leetcode.com/problems/block-placement-queries/submissions/2024212136/?envType=daily-question&envId=2026-06-05

struct Solution;

use std::collections::BTreeSet;

// power of two that will cover max gap position 50_000
const LEN: usize = 65_536;

struct SegmentTree {
    arr: Vec::<usize>
}

impl SegmentTree {
    pub fn new() -> Self {
        SegmentTree {
            arr: vec![0; LEN << 1],
        }
    }

    pub fn update(&mut self, location: usize, value: usize) {
        let mut i = location + LEN;
        self.arr[i] = value;
        i >>= 1;

        while i > 0 {
            let k = i << 1;
            let new = self.arr[k].max(self.arr[k + 1]);
            if self.arr[i] == new { break }
            self.arr[i] = new;
            i >>= 1;
        }
    }

    pub fn query(&self, left: usize, right: usize) -> usize {
        let mut res = 0;
        let mut left = left + LEN;
        let mut right = right + LEN;

        while left <= right {
            if left % 2 == 1 {
                res = res.max(self.arr[left]);
                left += 1;
            }
            if right % 2 == 0 {
                res = res.max(self.arr[right]);
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        res
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut res = vec![];
        let mut gaps = SegmentTree::new();
        let mut blocks = BTreeSet::<usize>::new();

        blocks.insert(0);
        blocks.insert(LEN - 1);
        gaps.update(LEN - 1, LEN - 1);

        for query in queries {
            let pos = query[1] as usize;
            let prev = *blocks.range(..pos).next_back().unwrap();

            match query[0] {
                1 => {
                    let next = *blocks.range(pos..).next().unwrap();
                    gaps.update(pos, pos - prev);
                    gaps.update(next, next - pos);
                    blocks.insert(pos);
                }
                _ => {
                    let needed = query[2] as usize;
                    let will_fit = gaps.query(0, prev).max(pos - prev) >= needed;
                    res.push(will_fit);
                }
            }
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::get_results(vec![
        vec![1,2],
        vec![2,3,3],
        vec![2,3,1],
        vec![2,2,2]
    ]);
    let expected = vec![false, true, true];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::get_results(vec![
        vec![1,7],
        vec![2,7,6],
        vec![1,2],
        vec![2,7,5],
        vec![2,7,6]
    ]);
    let expected = vec![true, true, false];
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::get_results(vec![
        vec![1,20],
        vec![1,10],
        vec![1,15],
        vec![1,2],
    ]);
    let expected = vec![];
    assert_eq!(actual, expected);
}
