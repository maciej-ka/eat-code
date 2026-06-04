// https://leetcode.com/problems/total-waviness-of-numbers-in-range-i/submissions/2022086521/?envType=daily-question&envId=2026-06-04

struct Solution;

impl Solution {
    fn waviness(num: i32) -> i32 {
        if num < 100 { return 0 }

        let mut res = 0;
        let mut num = num;
        // c b a
        let mut c;
        let mut b = 10;
        let mut a = 10;
        while num > 0  {
            c = b;
            b = a;
            a = num % 10;
            num /= 10;
            if c == 10 { continue }
            if (b > a && b > c) || (b < a && b < c) { res += 1; }
        }
        res
    }

    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut res = 0;
        for num in num1..=num2 {
            res += Solution::waviness(num);
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::total_waviness(120, 130);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::total_waviness(198, 202);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::total_waviness(4848, 4848);
    let expected = 2;
    assert_eq!(actual, expected);
}
