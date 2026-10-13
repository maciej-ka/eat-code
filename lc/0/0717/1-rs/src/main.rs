// https://leetcode.com/problems/1-bit-and-2-bit-characters/submissions/1833128793/

struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        if bits.len() == 1 { return true }
        let mut i = 0;
        while i < bits.len() - 1 {
            i += if bits[i] == 1 { 2 } else { 1 }
        }
        i == bits.len() - 1
    }
}

#[test]
fn test_1() {
    let actual = Solution::is_one_bit_character(vec![1, 0, 0]);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::is_one_bit_character(vec![1, 1, 1, 0]);
    let expected = false;
    assert_eq!(actual, expected);
}
