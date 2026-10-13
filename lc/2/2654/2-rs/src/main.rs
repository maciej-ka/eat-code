// https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/submissions/1827824406/

struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        let mut ones = 0;
        for n in &nums {
            if *n == 1 { ones += 1; }
        }
        if ones > 0 { return (len - ones) as i32 }

        for step in 1..len {
            for i in 0..len - step {
                let gcd = Self::gcd(nums[i], nums[i + 1]);
                if gcd == 1 { return (step + len - 1) as i32; }
                nums[i] = gcd;
            }
        }

        -1
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b > 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_operations(vec![2, 6, 3, 4]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_operations(vec![2, 10, 6, 14]);
    let expected = -1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_operations(vec![1, 10, 7, 11, 24, 2]);
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::min_operations(vec![1, 10, 7, 1, 24, 2]);
    let expected = 4;
    assert_eq!(actual, expected);
}
