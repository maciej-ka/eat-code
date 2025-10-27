// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/submissions/1812957225/

struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut result = 0;
        let mut last = 0;
        for row in bank {
            let mut count = 0;
            for char in row.chars() {
                if char == '1' { count += 1 }
            }
            if count > 0 {
                result += count * last;
                last = count;
            }
        }
        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::number_of_beams(vec![
        String::from("011001"),
        String::from("000000"),
        String::from("010100"),
        String::from("001000"),
    ]);
    let expected = 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::number_of_beams(vec![
        String::from("000"),
        String::from("111"),
        String::from("000"),
    ]);
    let expected = 0;
    assert_eq!(actual, expected);
}
