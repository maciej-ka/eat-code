// https://leetcode.com/problems/ones-and-zeroes/submissions/1827548913/

struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut elements = Vec::with_capacity(strs.len());
        for s in strs {
            let mut zeroes = 0;
            for c in s.chars() {
                if c == '0' { zeroes += 1; }
            }
            let ones = s.len() - zeroes;
            if zeroes <= m && ones <= n {
                elements.push((zeroes, ones))
            }
        }
        elements.sort();

        let mut dp = vec![vec![0; n + 1]; m + 1];
        for e in elements {
            for i in (0..=m).rev() {
                if i < e.0  { break }
                for j in (0..=n).rev() {
                    if j >= e.1 {
                        dp[i][j] = dp[i][j].max(1 + dp[i - e.0][j - e.1]);
                    }
                }
            }
        }

        dp[m][n]
    }
}

#[test]
fn test_1() {
    let actual = Solution::find_max_form(vec![String::from("10"), String::from("0001"), String::from("111001"), String::from("1"), String::from("0")], 5, 3);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::find_max_form(vec![String::from("10"), String::from("0"), String::from("1")], 1, 1);
    let expected = 2;
    assert_eq!(actual, expected);
}
