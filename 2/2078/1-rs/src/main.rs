// https://leetcode.com/problems/two-furthest-houses-with-different-colors/submissions/1983849691/?envType=daily-question&envId=2026-04-20
struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let ilast = colors.len() - 1;
        if colors[0] != colors[ilast] { return ilast as i32; }

        let mut iright = ilast;
        while colors[0] == colors[iright] { iright -= 1 }
        let mut ileft = 0;
        while colors[0] == colors[ileft] { ileft += 1 }

        iright.max(ilast - ileft) as i32
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
