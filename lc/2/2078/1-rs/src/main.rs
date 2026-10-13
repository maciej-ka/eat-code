// https://leetcode.com/problems/two-furthest-houses-with-different-colors/submissions/1983860121/?envType=daily-question&envId=2026-04-20
struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let ilast = colors.len() - 1;
        for i in 0..ilast {
            if colors[i] != colors[0] || colors[ilast - i] != colors[0] {
                return (ilast - i) as i32
            }
        }
        -1i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_distance(vec![1,1,1,6,1,1,1]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_distance(vec![1,8,3,8,3]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_distance(vec![0,1]);
    let expected = 1;
    assert_eq!(actual, expected);
}
