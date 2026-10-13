// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/submissions/1807615331/

struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let len = nums.len() as i32;
        nums.sort();

        let mut res: i32 = 1;
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut ex: i32 = 0; // exact
        let mut ex2: i32;

        for i in nums[0]..=nums[(len as usize) - 1] {
            ex2 = ex;
            while hi < len - 1 && nums[(hi + 1) as usize] <= i + k { hi += 1 }
            while nums[lo as usize] < i - k { lo += 1 }
            while ex2 < len - 1 && nums[(ex2 + 1) as usize] == i { ex2 += 1 }
            let range = hi - lo + 1;
            let repeats: i32 = ex2 - ex + if nums[ex as usize] == i { 1 } else { 0 };
            res = res.max(range.min(num_operations + repeats) as i32);
            ex = ex2;
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_frequency(vec![1,4,5], 1, 2);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_frequency(vec![5,11,20,20], 5, 1);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_frequency(vec![88,53], 27, 2);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::max_frequency(vec![37,30,37], 26, 1);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::max_frequency(vec![86,117,82,133,82], 32, 0);
    let expected = 2;
    assert_eq!(actual, expected);
}
