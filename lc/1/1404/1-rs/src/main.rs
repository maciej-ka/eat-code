// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/submissions/1932245683/?envType=daily-question&envId=2026-02-26

struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut ans = 0;
        let mut last = '0';
        let mut zeroes = 0;
        let mut ones = 0;

        for i in (0..chars.len()).rev() {
            let is_group_end = last == '0' && chars[i] == '1';

            if is_group_end {
                // tail
                if ones == 0 {
                    ans += zeroes;
                    zeroes = 0;

                // mid group
                } else {
                    ans += 1 + ones + ((zeroes - 1).max(0) << 1);
                    zeroes = 0;
                    ones = 1;
                }
            }

            last = chars[i];
            if chars[i] == '0' {
                zeroes += 1
            } else {
                ones += 1
            }
        }

        // head
        if ones > 1 {
            ans += 1 + ones;
        }

        ans
    }
}

#[test]
fn test_1() {
    let actual = Solution::num_steps(String::from("1101"));
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::num_steps(String::from("10"));
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::num_steps(String::from("1"));
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::num_steps(String::from("1010"));
    let expected = 6;
    assert_eq!(actual, expected);
}
