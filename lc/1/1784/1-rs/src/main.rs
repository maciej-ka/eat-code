// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/submissions/1939555805/?envType=daily-question&envId=2026-03-06

struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let bytes: Vec<u8> = s.bytes().collect();
        let mut last = bytes[0];
        let mut count = if *(bytes.last().unwrap()) == b'1' { 1 } else { 0 };

        for b in bytes {
            if last == b'1' && b == b'0' { count += 1 }
            last = b;
        }
        count < 2
    }
}


#[test]
fn test_1() {
    let actual = Solution::check_ones_segment(String::from("1001"));
    let expected = false;
    assert_eq!(actual, expected);
}


#[test]
fn test_2() {
    let actual = Solution::check_ones_segment(String::from("110"));
    let expected = true;
    assert_eq!(actual, expected);
}
