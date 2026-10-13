struct Solution;

impl Solution {
    fn solve(nums: Vec<i32>) -> usize {
        nums.len()
    }
}

#[test]
fn test_1() {
    let actual = Solution::solve(vec![1, 2, 3]);
    let expected = 3;
    assert_eq!(actual, expected);
}
