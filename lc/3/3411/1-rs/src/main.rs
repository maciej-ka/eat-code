// https://leetcode.com/problems/maximum-subarray-with-equal-products/submissions/2204209683/?envType=daily-question&envId=2026-11-26

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let last = nums.len() - 1;
        let mut best = 2;
        let mut left = 0;
        let mut right = 1;

        while best < last - left + 1 && right <= last {
            let mut coprime = true;
            for i in (left..right).rev() {
                if Self::gcp(nums[i], nums[right]) != 1 {
                    left = i + 1;
                    coprime = false;
                    break;
                }
            }

            if coprime { best = best.max(right - left + 1) }
            right += 1;
        }

        best as i32
    }

    pub fn gcp(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_length(vec![1,2,1,2,1,1,1]);
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_length(vec![2,3,4,5,6]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_length(vec![2,6]);
    let expected = 2;
    assert_eq!(actual, expected);
}
