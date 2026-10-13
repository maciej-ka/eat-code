// https://leetcode.com/problems/keep-multiplying-found-values-by-two/submissions/1834040560/

struct Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut lookup = [false; 1001];
        for n in nums { lookup[n as usize] = true }

        let mut res = original;
        while res <= 1000 && lookup[res as usize] { res <<= 1 }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::find_final_value(vec![5,3,6,1,12], 3);
    let expected = 24;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::find_final_value(vec![2,7,9], 4);
    let expected = 4;
    assert_eq!(actual, expected);
}
