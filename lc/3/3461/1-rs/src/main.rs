// https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i/submissions/1809350594/

struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut vec: Vec<u32> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut k = vec.len() - 1;

        while k > 1 {
            for i in 0..k {
                vec[i] = (vec[i] + vec[i + 1]) % 10;
            }
            k -= 1;
        }

        vec[0] == vec[1]
    }
}

#[test]
fn test_1() {
    let actual = Solution::has_same_digits(String::from("3902"));
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::has_same_digits(String::from("34789"));
    let expected = false;
    assert_eq!(actual, expected);
}
