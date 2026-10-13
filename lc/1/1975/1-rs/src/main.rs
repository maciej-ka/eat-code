// https://leetcode.com/problems/maximum-matrix-sum/submissions/1875401144/

struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0i64;
        let mut even = true;
        let mut min = 100001i64;

        for i in 0..matrix.len() {
            for k in 0..matrix[i].len() {
                let val = matrix[i][k] as i64;
                if val < 0 { even = !even }
                sum += val.abs();
                min = min.min(val.abs())
            }
        }

        match even {
            true => sum,
            false => sum - (min << 1)
        }
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_matrix_sum(vec![vec![1,2,3], vec![-1,-2,-3], vec![1,2,3]]);
    let expected = 16;
    assert_eq!(actual, expected);
}
