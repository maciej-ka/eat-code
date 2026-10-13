// https://leetcode.com/problems/make-array-elements-equal-to-zero/submissions/1813760886/

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let len: i32 = nums.len().try_into().unwrap();

        let sum = nums.iter().sum();
        if sum == 0 { return len << 1}
        if sum == 1 { return len - 1 }

        let mut r: i32 = sum;
        let mut l: i32 = 0;
        let mut res = 0;

        for i in 0..nums.len() {

            let val = nums[i];

            if l > r + 1 { break }

            if val == 0 {
                let d = (r - l).abs();
                if d == 0 { res += 2 }
                if d == 1 { res += 1 }
                continue;
            }

            l += val;
            r -= val;
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::count_valid_selections(vec![1, 0, 2, 0, 3]);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]);
    let expected = 0;
    assert_eq!(actual, expected);
}
