// https://leetcode.com/problems/smallest-integer-divisible-by-k/submissions/1839286489/

struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let k = k as usize;
        if k & 1 == 0 { return -1 }
        let mut res = 1;
        let mut m = 1 % k;
        let mut seen = vec![0; k];
        while m != 0 {
            if seen[m] == 1 { return -1 }
            seen[m] = 1;
            m = (m * 10 + 1) % k;
            res += 1;
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::smallest_repunit_div_by_k(1);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::smallest_repunit_div_by_k(2);
    let expected = -1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::smallest_repunit_div_by_k(3);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::smallest_repunit_div_by_k(7);
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::smallest_repunit_div_by_k(5);
    let expected = -1;
    assert_eq!(actual, expected);
}

#[test]
fn test_6() {
    let actual = Solution::smallest_repunit_div_by_k(11);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_7() {
    let actual = Solution::smallest_repunit_div_by_k(15);
    let expected = -1;
    assert_eq!(actual, expected);
}
