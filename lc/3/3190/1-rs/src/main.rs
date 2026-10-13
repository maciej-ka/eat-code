// https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/submissions/1836522112/

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            if n % 3 > 0 {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::minimum_operations(vec![1,2,3,4]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::minimum_operations(vec![3,6,9]);
    let expected = 0;
    assert_eq!(actual, expected);
}
