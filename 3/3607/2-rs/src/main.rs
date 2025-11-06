// https://leetcode.com/problems/power-grid-maintenance/submissions/1822824117

struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;

        let mut groups = Vec::<usize>::with_capacity(c + 1);
        for i in 0..c + 1 { groups.push(i) }

        // union
        for c in connections {
            let v = c[0] as usize;
            let u = c[1] as usize;
            let uid = Self::find_id(u, &mut groups);
            let vid = Self::find_id(v, &mut groups);
            groups[vid] = uid;
            groups[v] = uid;
        }

        for i in 0..groups.len() {
            groups[i] = Self::find_id(i, &mut groups);
        }

        // map owns sets
        let mut sets = HashMap::<usize, BTreeSet<usize>>::new();
        for i in 1..c + 1 {
            let id = Self::find_id(i, &mut groups);
            let set = sets.entry(id).or_insert(BTreeSet::new());
            set.insert(i);
        }

        let mut res = Vec::new();
        for q in &queries {
            let num = q[1] as usize;
            let id = groups[num];

            // turn off
            if q[0] == 2 {
                sets.entry(id).and_modify(|set| { set.remove(&num); });
                continue;
            }

            // check
            let set = sets.get(&id).unwrap();
            if set.contains(&num) {
                res.push(num as i32);
                continue
            }

            if let Some(first) = set.first() {
                res.push(*first as i32);
                continue
            }

            res.push(-1)
        }

        res
    }

    fn find_id(i: usize, groups: &mut Vec<usize>) -> usize {
        if groups[i] == i { return i }
        groups[i] = Self::find_id(groups[i], groups);
        groups[i]
    }
}

#[test]
fn test_1() {
    let actual = Solution::process_queries(5, vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]], vec![vec![1,3],vec![2,1],vec![1,1],vec![2,2],vec![1,2]]);
    let expected = vec![3,2,3];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::process_queries(3, vec![], vec![vec![1,1],vec![2,1],vec![1,1]]);
    let expected = vec![1,-1];
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::process_queries(5, vec![vec![1,3],vec![3,5],vec![2,4]], vec![vec![1,3],vec![2,3],vec![1,3],vec![2,1],vec![1,3],vec![2,5],vec![1,3]]);
    let expected = vec![3,1,5,-1];
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::process_queries(4, vec![vec![3,4],vec![4,2]], vec![vec![1,1],vec![2,3],vec![2,2],vec![2,1],vec![1,1],vec![2,2],vec![1,1],vec![2,1],vec![1,4],vec![1,3]]);
    let expected = vec![1,-1,-1,4,4];
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::process_queries(4, vec![vec![4,3],vec![3,1],vec![4,2],vec![3,2],vec![4,1]], vec![vec![2,3],vec![1,2],vec![2,4],vec![1,1],vec![2,2],vec![1,2],vec![1,2],vec![2,2],vec![1,3],vec![2,3],vec![2,4],vec![2,3],vec![2,4],vec![1,2],vec![1,1]]);
    let expected = vec![2,1,1,1,1,1,1];
    assert_eq!(actual, expected);
}
