// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/?envType=daily-question&envId=2026-03-03

struct Solution;

impl Solution {
    fn solve(n: i32, k: i32) -> i32 {
        if n == 1 { return 0 }
        let last = (2 << n - 1) - 2;
        let half = last / 2;
        if k == half { return 1 }
        if k < half { return Self::solve(n - 1, k) }
        (Self::solve(n - 1, last - k) + 1) % 2
    }

    pub fn find_kth_bit(n: i32, k: i32) -> char {
        match Self::solve(n, k - 1) {
            0 => '0',
            _ => '1'
        }
    }
}

#[test]
fn test_1() {
    let actual = Solution::find_kth_bit(3, 1);
    let expected = '0';
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::find_kth_bit(4, 11);
    let expected = '1';
    assert_eq!(actual, expected);
}
