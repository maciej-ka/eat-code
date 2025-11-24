// https://leetcode.com/problems/binary-prefix-divisible-by-5/submissions/1838355629/

struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut res = Vec::<bool>::with_capacity(nums.len());
        let mut m = 0;

        for n in nums {
            m = ((m << 1) + n) % 5;
            res.push(m == 0);
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::prefixes_div_by5(vec![0, 1, 1]);
    let expected = vec![true, false, false];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::prefixes_div_by5(vec![1, 1, 1]);
    let expected = vec![false, false, false];
    assert_eq!(actual, expected);
}
