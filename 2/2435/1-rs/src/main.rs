// https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/submissions/1840180803/

struct Solution;

const MOD: i32 = 1000000007;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut memo = vec![vec![vec![0; k]; cols]; rows];
        memo[0][0][grid[0][0] as usize % k] = 1;

        for i in 0..rows {
            for j in 0..cols {
                let val = grid[i][j] as usize;
                if i > 0 {
                    for l in 0..k {
                        memo[i][j][(l + val) % k] = memo[i - 1][j][l] % MOD;
                    }
                }
                if j > 0 {
                    for l in 0..k {
                        memo[i][j][(l + val) % k] += memo[i][j - 1][l] % MOD;
                    }
                }
            }
        }
        memo[rows - 1][cols - 1][0] % MOD
    }
}

#[test]
fn test_1() {
    let actual = Solution::number_of_paths(vec![vec![5,2,4],vec![3,0,5],vec![0,7,2]], 3);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::number_of_paths(vec![vec![0,0]], 5);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::number_of_paths(vec![vec![7,3,4,9],vec![2,3,6,2],vec![2,3,7,0]], 1);
    let expected = 10;
    assert_eq!(actual, expected);
}

