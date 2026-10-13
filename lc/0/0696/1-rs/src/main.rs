// https://leetcode.com/problems/count-binary-substrings/submissions/1924853292/?envType=daily-question&envId=2026-02-19

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut vec = vec![];
        let mut last = 'x';
        let mut count = 0;

        for c in s.chars() {
            if last != 'x' && last != c {
                vec.push(count);
                count = 0;
            }
            count += 1;
            last = c;
        }
        vec.push(count);

        let mut ans = 0;
        for i in 0..vec.len() - 1 {
            ans += vec[i].min(vec[i + 1])
        }

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
