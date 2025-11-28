// https://leetcode.com/problems/maximum-number-of-k-divisible-components/submissions/1842055856/

struct Solution;

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, mut values: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;

        for i in 0..n {
            values[i] %= k;
        }

        let mut conn = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            conn[u].push(v);
            conn[v].push(u);
        }

        let mut res = 0;
        let mut visited = vec![false; n];
        let mut stack = vec![];

        stack.push(0);
        visited[0] = true;

        while stack.len() > 0 {
            let current = stack[stack.len() - 1];

            // unfold
            if let Some(i) = conn[current].pop() {
                if visited[i] { continue }
                stack.push(i);
                visited[i] = true;
                continue;
            }

            // fold
            stack.pop();
            if values[current] == 0 { res += 1 }
            if stack.len() > 0 {
                let p = stack[stack.len() - 1];
                values[p] = (values[p] + values[current]) % k;
            }
        }

        res
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
