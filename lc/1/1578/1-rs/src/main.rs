// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/submissions/1819673013/

struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut last: char = colors.chars().next().unwrap();
        let mut max = 0;
        let mut sum = 0;
        for (i, c) in colors.char_indices() {
            if c != last {
                res += sum - max;
                sum = 0;
                max = 0;
                last = c;
            }
            max = max.max(needed_time[i]);
            sum += needed_time[i];
        }
        res + sum - max
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_cost(String::from("abaac"), vec![1, 2, 3, 4, 5]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_cost(String::from("abc"), vec![1, 2, 3]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_cost(String::from("aabaa"), vec![1, 2, 3, 4, 1]);
    let expected = 2;
    assert_eq!(actual, expected);
}
