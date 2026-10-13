// https://leetcode.com/problems/greatest-sum-divisible-by-three/submissions/1837799495/

struct Solution;

const MAX: i32 = 10001;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        // minimal value
        let mut mod1a = MAX;
        let mut mod2a = MAX;
        // second value
        let mut mod1b = MAX;
        let mut mod2b = MAX;

        let mut sum = 0;
        for n in nums {
            sum += n;
            let m = n % 3;
            if m == 1 {
                if n < mod1a {
                    mod1b = mod1a;
                    mod1a = n;
                } else if n < mod1b {
                    mod1b = n;
                }
            } else if m == 2 {
                if n < mod2a {
                    mod2b = mod2a;
                    mod2a = n;
                } else if n < mod2b {
                    mod2b = n;
                }
            }
        }

        let m = sum % 3;
        if m == 0 { return sum }
        if m == 1 {
            let d = mod1a.min(mod2a + mod2b);
            return if d < MAX { sum - d } else { 0 }
        }
        if m == 2 {
            let d = mod2a.min(mod1a + mod1b);
            return if d < MAX { sum - d } else { 0 }
        }
        0
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_sum_div_three(vec![3,6,5,1,8]);
    let expected = 18;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_sum_div_three(vec![4]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_sum_div_three(vec![1,2,3,4,4]);
    let expected = 12;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::max_sum_div_three(vec![1,2,3]);
    let expected = 6;
    assert_eq!(actual, expected);
}
