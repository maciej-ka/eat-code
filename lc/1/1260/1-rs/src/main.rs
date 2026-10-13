// https://leetcode.com/problems/shift-2d-grid/submissions/2074737415/?envType=daily-question&envId=2026-07-20

struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0; n];m];

        let total = m * n;
        let mut i = total - k as usize % total;
        for k in 0..total {
            i = i % total;
            ans[k / n][k % n] = grid[i / n][i % n];
            i += 1;
        }

        ans
    }
}

#[test]
fn test_1() {
    let actual = Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1);
    let expected = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::shift_grid(vec![vec![3, 8, 1, 9],vec![19, 7, 2, 5],vec![4, 6, 11, 10],vec![12, 0, 21, 13]], 4);
    let expected = vec![vec![12, 0, 21, 13],vec![3, 8, 1, 9],vec![19, 7, 2, 5],vec![4, 6, 11, 10]];
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9);
    let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::shift_grid(vec![vec![1],vec![2],vec![3],vec![4],vec![7],vec![6],vec![5]], 23);
    let expected = vec![vec![6],vec![5],vec![1],vec![2],vec![3],vec![4],vec![7]];
    assert_eq!(actual, expected);
}
