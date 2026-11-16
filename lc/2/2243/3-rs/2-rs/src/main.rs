// https://leetcode.com/problems/calculate-digit-sum-of-a-string/submissions/2197565781/?envType=daily-question&envId=2026-11-16
struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s = s;

        while s.len() > k {
            let mut round = String::with_capacity(s.len());
            let mut sum = 0;
            for (i, char) in s.chars().enumerate() {
                sum += char.to_digit(10).unwrap();
                if (i + 1) % k == 0 || i == s.len() - 1 {
                    round.push_str(&sum.to_string());
                    sum = 0;
                }
            }
            s = round;
        }

        s
    }
}

#[test]
fn test_1() {
    let actual = Solution::digit_sum(String::from("11111222223"), 3);
    let expected = String::from("135");
    assert_eq!(actual, expected);
}
