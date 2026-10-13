// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/submissions/1946502459/?envType=daily-question&envId=2026-03-12

struct Solution;

use std::{cmp::Reverse, collections::{BinaryHeap, HashSet, VecDeque}};

// 0: str, 1: node, 2: me, 3: mandatory
type Conn = (i32, usize, usize, bool);

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;

        let mut conns: Vec<Vec<Conn>> = vec![vec![]; n];
        for i in 0..edges.len() {
            let edge = &edges[i];
            conns[edge[0] as usize].push((edge[2], edge[1] as usize, edge[0] as usize, edge[3] == 1));
            conns[edge[1] as usize].push((edge[2], edge[0] as usize, edge[1] as usize, edge[3] == 1));
        }

        let mut visited = HashSet::<usize>::new();
        let mut queue = VecDeque::<&Conn>::new();
        let mut heap = BinaryHeap::<&Conn>::new();
        let mut str_heap = BinaryHeap::<Reverse<&Conn>>::new();
        let mut ans = i32::MAX;

        queue.push_back(&(i32::MAX, 0, 0, true));

        while visited.len() < n || queue.len() > 0 {

            if !queue.is_empty() {
                let conn = queue.pop_front().unwrap();

                // mark as visited
                if visited.contains(&conn.1) {
                    if conn.3 { return -1; } else { continue };
                }
                visited.insert(conn.1);

                // record strength
                if conn.3 {
                    ans = ans.min(conn.0)
                } else {
                    str_heap.push(Reverse(conn));
                }

                // follow connections
                for c in &conns[conn.1] {
                    if c.1 == conn.2 { continue }
                    if c.3 { queue.push_back(c) }
                    else { heap.push(c); }
                }
                continue;
            }

            if heap.is_empty() { return -1; }
            queue.push_back(heap.pop().unwrap());
        }

        let times = k.min(str_heap.len());
        for _i in 0..times {
            ans = ans.min(str_heap.pop().unwrap().0.0 << 1);
        }
        if str_heap.len() > 0 {
            ans = ans.min(str_heap.pop().unwrap().0.0);
        }

        ans
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_stability(3, vec![vec![0,1,2,1], vec![1,2,3,0]], 1);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_stability(3, vec![vec![0,1,4,0], vec![1,2,3,0], vec![0,2,1,0]], 2);
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_stability(3, vec![vec![0,1,1,1], vec![1,2,1,1], vec![2,0,1,1]], 0);
    let expected = -1;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::max_stability(6, vec![vec![4,5,8,1], vec![1,2,2,0], vec![4,3,3,0], vec![2,0,1,0], vec![2,3,8,1], vec![3,1,2,0], vec![1,0,8,1], vec![0,3,1,0]], 2);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::max_stability(2, vec![vec![0,1,87487,0]], 0);
    let expected = 87487;
    assert_eq!(actual, expected);
}

