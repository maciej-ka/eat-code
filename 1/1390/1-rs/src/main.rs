// https://leetcode.com/problems/four-divisors/submissions/1874117313

struct Solution;

const MAX: usize = 100000;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut sums = [1; MAX + 1];
        let mut counts = [1; MAX + 1];

        for i in 2..=MAX {
            let mut n = i;
            while n <= MAX {
                counts[n] += 1;
                if counts[n] <= 4 {
                    sums[n] += i;
                }
                n += i;
            }
        }

        let mut res = 0;
        for i in 0..nums.len() {
            let n = nums[i] as usize;
            if counts[n] == 4 {
                res += sums[n];
            }
        }

        res as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::sum_four_divisors(vec![21, 4, 7]);
    let expected = 32;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::sum_four_divisors(vec![21, 21]);
    let expected = 64;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]);
    let expected = 0;
    assert_eq!(actual, expected);
}
