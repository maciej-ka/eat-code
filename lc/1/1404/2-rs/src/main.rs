// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/submissions/1932253035/?envType=daily-question&envId=2026-02-26
struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut ans = 0;
        let mut i = chars.len() - 1;

        while chars[i] == '0' {
            ans += 1;
            i -= 1;
        }

        if i == 0 {
            return ans
        } else {
            ans += 2 + i as i32
        }

        loop {
            if chars[i] == '0' {
                ans += 1
            }
            if i == 0 { break }
            i -= 1;
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
