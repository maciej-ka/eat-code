// https://leetcode.com/problems/power-grid-maintenance/submissions/1822716041/

struct Solution;

use std::collections::{BTreeSet, HashMap, HashSet};

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;

        let mut connected = vec![HashSet::new(); c + 1];
        for c in connections {
            connected[c[0] as usize].insert(c[1] as usize);
            connected[c[1] as usize].insert(c[0] as usize);
        }

        // map owns sets
        let mut sets = HashMap::<usize, BTreeSet<usize>>::new();
        let mut ids = Vec::<usize>::with_capacity(c + 1);
        for i in 0..c + 1 { ids.push(i) }
        let mut visited = HashSet::new();

        for i in 1..=c {
            if visited.contains(&i) { continue }

            let mut set = BTreeSet::new();
            let mut stack = Vec::new();
            stack.push(i);

            while let Some(node) = stack.pop() {
                if visited.contains(&node) { continue }
                set.insert(node);
                visited.insert(node);
                for c in &connected[node] { stack.push(*c) }
            }

            let first = set.first().unwrap();
            for node in &set {
                ids[*node] = *first;

            }
            ids[i] = *set.first().unwrap();
            sets.insert(i, set);
        }

        let mut res = Vec::new();
        for q in &queries {
            let num = q[1] as usize;
            let id = ids[num];

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
