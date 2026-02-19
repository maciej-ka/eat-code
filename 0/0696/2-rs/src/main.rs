// https://leetcode.com/problems/count-binary-substrings/submissions/1924862299/?envType=daily-question&envId=2026-02-19

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut ans = 0;
        let mut last = ' ';
        let mut count1 = 0;
        let mut count2 = 0;

        for c in s.chars() {
            if last != c {
                ans += count1.min(count2);
                count2 = count1;
                count1 = 0;
            }
            count1 += 1;
            last = c;
        }
        ans += count1.min(count2);
        ans
    }
}

#[test]
fn test_1() {
    let actual = Solution::count_binary_substrings(String::from("00110011"));
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::count_binary_substrings(String::from("10101"));
    let expected = 4;
    assert_eq!(actual, expected);
}
