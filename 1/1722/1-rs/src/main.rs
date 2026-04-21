// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/submissions/1984501848/?envType=daily-question&envId=2026-04-21

struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let mut conn = HashMap::<usize, Vec<usize>>::new();
        for i in 0..allowed_swaps.len() {
            let u = allowed_swaps[i][0] as usize;
            let v = allowed_swaps[i][1] as usize;
            conn.entry(u).and_modify(|vec| vec.push(v)).or_insert(vec![v]);
            conn.entry(v).and_modify(|vec| vec.push(u)).or_insert(vec![u]);
        }

        let mut visited = HashSet::<usize>::new();
        let mut stack = VecDeque::<usize>::new();
        let mut result = 0;
        for i in 0..source.len() {
            if visited.contains(&i) { continue }

            let mut counts_src = HashMap::<usize, usize>::new();
            let mut counts_tar = HashMap::<usize, usize>::new();
            stack.push_front(i);

            while let Some(k) = stack.pop_back() {
                if visited.contains(&k) { continue }
                counts_src.entry(source[k] as usize).and_modify(|c| *c += 1).or_insert(1);
                counts_tar.entry(target[k] as usize).and_modify(|c| *c += 1).or_insert(1);
                visited.insert(k);
                if let Some(list) = conn.get(&k) {
                    for c in list {
                        stack.push_front(*c);
                    }
                }
            }

            for (key, c_src) in counts_src {
                let c_tar = counts_tar.get(&key).copied().unwrap_or_default();
                if c_src > c_tar {
                    result += c_src - c_tar;
                }
            }
        }

        result as i32
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
