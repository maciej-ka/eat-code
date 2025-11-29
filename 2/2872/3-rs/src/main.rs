// https://leetcode.com/problems/maximum-number-of-k-divisible-components/submissions/1842470963/

struct Solution;

struct Result {
    sum: i32,
    cuts: i32,
}

fn visit(i: usize, values: &Vec<i32>, conn: &Vec<Vec<usize>>, visited: &mut Vec<bool>, k: i32) -> Result {
    visited[i] = true;
    let mut sum = values[i];
    let mut cuts = 0;
    for c in &conn[i] {
        if visited[*c] { continue }
        let res = visit(*c, values, conn, visited, k);
        cuts += res.cuts;
        sum += res.sum;
        if res.sum == 0 { cuts += 1 }
    }
    Result { sum: sum % k, cuts }
}

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;

        let mut conn = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            conn[u].push(v);
            conn[v].push(u);
        }

        let mut visited = vec![false; n];
        visit(0, &values, &conn, &mut visited, k).cuts + 1
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_k_divisible_components(5, vec![vec![0,2],vec![1,2],vec![1,3],vec![2,4]], vec![1,8,1,4,4], 6);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_k_divisible_components(7, vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![2,6]], vec![3,0,6,1,5,2,1], 3);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_k_divisible_components(9, vec![vec![1,2],vec![1,7],vec![0,6],vec![0,8],vec![0,3],vec![3,4],vec![0,5],vec![2,5]], vec![1,4,4,0,2,1,1,6,2], 7);
    let expected = 2;
    assert_eq!(actual, expected);
}
