// https://leetcode.com/problems/number-of-substrings-with-only-1s/submissions/1831192283/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut lens = HashMap::<i32, i32>::new();
        let mut len = 0;
        for c in s.bytes() {
            if c == b'1' {
                len += 1;
            } else if len > 0 {
                lens.entry(len).and_modify(|x| *x += 1).or_insert(1);
                len = 0;
            }
        }
        if len > 0 {
            lens.entry(len).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut res = 0;
        for (len, count) in lens {
            let sum;
            if len & 1 == 0 {
                sum = Self::fmm(len >> 1, len + 1)
            } else {
                sum = Self::fmm(len, len + 1 >> 1)
            }
            res = (res + Self::fmm(sum, count)) % Self::MOD;
        }

        res
    }

    const MOD: i32 = 1e9 as i32 + 7;

    fn fmm(a: i32, b: i32) -> i32 {
        let mut res = 0;
        let mut b = b;
        let mut add = a;
        while b > 0 {
            if b & 1 > 0 { res = (res + add) % Self::MOD }
            b >>= 1;
            add = (add << 1) % Self::MOD
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::num_sub(String::from("0110111"));
    let expected = 9;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::num_sub(String::from("101"));
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::num_sub(String::from("111111"));
    let expected = 21;
    assert_eq!(actual, expected);
}
