struct Solution;

impl Solution {
    fn solve(arr: &[i32]) -> usize {
        arr.len()
    }
}

#[test]
fn test_1() {
    let actual = Solution::solve(&[1, 2, 3]);
    let expected = 3;
    assert_eq!(actual, expected);
}
