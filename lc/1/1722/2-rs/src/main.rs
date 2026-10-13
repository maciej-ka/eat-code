// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/submissions/1984805796/?envType=daily-question&envId=2026-04-21

struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find(index: usize, groups: &mut Vec<usize>) -> usize {
        if groups[index] == index { return index };
        groups[index] = Self::find(groups[index], groups);
        groups[index]
    }

    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let len = source.len();
        let mut groups = Vec::<usize>::with_capacity(len);
        for i in 0..len { groups.push(i) }

        for i in 0..allowed_swaps.len() {
            let a = Self::find(allowed_swaps[i][0] as usize, &mut groups);
            let b = Self::find(allowed_swaps[i][1] as usize, &mut groups);
            groups[a] = b
        }

        let mut buckets_src = HashMap::<usize, HashMap<i32, usize>>::new();
        let mut buckets_trg = HashMap::<usize, HashMap<i32, usize>>::new();
        for i in 0..len {
            let id = Self::find(i, &mut groups);
            let bucket_src = buckets_src.entry(id).or_insert(HashMap::new());
            let bucket_trg = buckets_trg.entry(id).or_insert(HashMap::new());
            bucket_src.entry(source[i]).and_modify(|x| *x += 1).or_insert(1);
            bucket_trg.entry(target[i]).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut result = 0;
        for (id, bucket_src) in buckets_src {
            let bucket_trg = buckets_trg.get(&id).unwrap();
            for (num, count_src) in bucket_src {
                let count_trg = bucket_trg.get(&num).copied().unwrap_or_default();
                if count_src > count_trg {
                    result += count_src - count_trg;
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

#[test]
fn test_4() {
    let actual = Solution::minimum_hamming_distance(vec![2, 3, 1], vec![1, 2, 2], vec![vec![0, 2], vec![1, 2]]);
    let expected = 1;
    assert_eq!(actual, expected);
}

