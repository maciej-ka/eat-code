// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/submissions/1947467269/?envType=daily-question&envId=2026-03-13

struct Solution;

// 0: strenght, 1: u, 2: v
type Edge = (i32, usize, usize);

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut groups = Groups::new(n);
        let mut ans = i32::MAX;

        let mut optional: Vec<Edge> = vec![];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            match edge[3] {
                1 => {
                    if !groups.union(u, v) { return -1 }
                    ans = ans.min(edge[2]);
                }
                _ => optional.push((edge[2], u, v)),
            }
        }

        if groups.count == n - 1 { return ans }
        optional.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        for opt in optional {
            if !groups.union(opt.1, opt.2) { continue }
            let strenght = if k >= n - groups.count { opt.0 << 1 } else { opt.0 };
            ans = ans.min(strenght);
            if groups.count == n - 1 { return ans }
        }

        -1
    }
}

struct Groups {
    parent: Vec<usize>,
    count: usize
}

impl Groups {
    pub fn new(n: usize) -> Self {
        let mut s = Groups {
            parent: Vec::<usize>::with_capacity(n),
            count: 0
        };
        for i in 0..n { s.parent.push(i); }
        s
    }

    pub fn find(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u])
        };
        self.parent[u]
    }

    pub fn union(&mut self, u: usize, v: usize) -> bool {
        let parent_u = self.find(u);
        let parent_v = self.find(v);
        if parent_u == parent_v { return false }
        self.parent[parent_u] = parent_v;
        self.count += 1;
        return true
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

