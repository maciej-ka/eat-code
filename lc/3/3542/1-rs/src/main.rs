// https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/submissions/1825924895/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut stack = Vec::new();
        for n in nums {
            // clear monotonic stack
            loop {
                if *stack.last().unwrap_or(&0) <= n { break }
                stack.pop();
                res += 1;
            }
            if n > *stack.last().unwrap_or(&0) {
                stack.push(n);
            }
        }

        return res + stack.len() as i32;
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_operations(vec![0, 2]);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_operations(vec![3, 1, 2, 1]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_operations(vec![1, 2, 1, 2, 1, 2]);
    let expected = 4;
    assert_eq!(actual, expected);
}
