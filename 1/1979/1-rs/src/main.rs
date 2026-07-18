struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut min = nums[0];
        for n in nums {
            max = n.max(max);
            min = n.min(min);
        }

        while min != 0 {
            let diff = max - min;
            max = diff.max(min);
            min = diff.min(min);
        }

        max
    }
}

#[test]
fn test_1() {
    let actual = Solution::find_gcd(vec![2, 5, 6, 9, 10]);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::find_gcd(vec![7, 5, 6, 8, 3]);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::find_gcd(vec![3, 3]);
    let expected = 3;
    assert_eq!(actual, expected);
}
