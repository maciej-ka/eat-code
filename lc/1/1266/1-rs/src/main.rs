// https://leetcode.com/problems/minimum-time-visiting-all-points/submissions/1883053742/?envType=daily-question&envId=2026-01-12

struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..points.len() - 1 {
            let a = &points[i];
            let b = &points[i + 1];
            res += (a[0] - b[0]).abs().max((a[1] - b[1]).abs())
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_time_to_visit_all_points(vec![vec![1,1],vec![3,4],vec![-1,0]]);
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_time_to_visit_all_points(vec![vec![3,2],vec![-2,2]]);
    let expected = 5;
    assert_eq!(actual, expected);
}
