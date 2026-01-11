// https://leetcode.com/problems/maximal-rectangle/submissions/1881805914/?envType=daily-question&envId=2026-01-11
struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut heights = vec![vec![0; cols]; rows];

        for j in 0..cols {
            let mut sum = 0;
            for i in (0..rows).rev() {
                let val = matrix[i][j];
                sum = if val == '0' { 0 } else { sum + 1 };
                heights[i][j] = sum;
            }
        }

        let mut best = 0;

        for i in 0..rows {
            for j in 0..cols {
                let mut h = heights[i][j];
                for k in j..cols {
                    if matrix[i][k] == '0' { break; }
                    let w = k - j + 1;
                    h = h.min(heights[i][k]);
                    best = best.max(w * h);
                }
            }
        }

        best as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::maximal_rectangle(vec![
        vec!['1','0','1','0','0'],
        vec!['1','0','1','1','1'],
        vec!['1','1','1','1','1'],
        vec!['1','0','0','1','0']
    ]);
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::maximal_rectangle(vec![vec!['0']]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::maximal_rectangle(vec![vec!['1']]);
    let expected = 1;
    assert_eq!(actual, expected);
}
