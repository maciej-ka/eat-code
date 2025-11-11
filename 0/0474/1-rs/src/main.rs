// https://leetcode.com/problems/ones-and-zeroes/submissions/1826937874/

struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut elements = Vec::with_capacity(strs.len());
        for s in strs {
            let mut zeroes = 0;
            let mut ones = 0;
            for c in s.chars() {
                if c == '0' {
                    zeroes += 1;
                } else {
                    ones += 1;
                }
            }
            if zeroes <= m && ones <= n {
                elements.push((zeroes, ones))
            }
        }
        elements.sort();

        let mut dp = vec![vec![vec![0; elements.len() + 1]; n + 1]; m + 1];
        for i in 0..= m {
            for j in 0..= n {
                for k in 0..elements.len() {
                    if elements[k].0 <= i && elements[k].1 <= j {
                        dp[i][j][k + 1] = dp[i][j][k].max(1 + dp[i - elements[k].0][j - elements[k].1][k]);
                    } else {
                        dp[i][j][k + 1] = dp[i][j][k];
                    }
                }
            }
        }

        dp[m][n][elements.len()]
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
