// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/submissions/1984828399/?envType=daily-question&envId=2026-04-21

struct Solution;

use std::collections::HashMap;

struct UnionFind {
    groups: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self { groups: (0..size).collect() }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.groups[i] != i { self.groups[i] = self.find(self.groups[i]) }
        self.groups[i]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let parent_a = self.find(a);
        let parent_b = self.find(b);
        self.groups[parent_b] = parent_a;
        self.groups[a] = parent_a;
    }
}

impl Solution {
    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let len = source.len();
        let mut groups = UnionFind::new(len);
        for edge in allowed_swaps { groups.union(edge[0] as usize, edge[1] as usize) }

        let mut buckets = vec![HashMap::<i32, i32>::new(); len];
        for i in 0..len {
            let bucket = buckets.get_mut(groups.find(i)).unwrap();
            bucket.entry(source[i]).and_modify(|x| *x += 1).or_insert(1);
            bucket.entry(target[i]).and_modify(|x| *x -= 1).or_insert(-1);
        }

        let mut result = 0;
        for bucket in buckets {
            for val in bucket.values() {
                if *val > 0 {
                    result += val
                }
            }
        }

        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![2, 1, 4, 5], vec![vec![0, 1], vec![2, 3]]);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec![]);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::minimum_hamming_distance(vec![5, 1, 2, 4, 3], vec![1, 5, 4, 2, 3], vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::minimum_hamming_distance(vec![2, 3, 1], vec![1, 2, 2], vec![vec![0, 2], vec![1, 2]]);
    let expected = 1;
    assert_eq!(actual, expected);
}

