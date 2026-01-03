// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/submissions/1873094011

struct Solution;

const MOD: i64 = 1e9 as i64 + 7;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let n = n as i32;
        let mut colors2 = 6 as i64;
        let mut colors3 = 6 as i64;

        for i in 1..n {
            let mut temp = colors2 * 3 % MOD + (colors3 << 1) % MOD;
            colors3 = (colors2 << 1) % MOD + (colors3 << 1) % MOD;
            colors2 = temp;
        }

        ((colors2 + colors3) % MOD) as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::num_of_ways(1);
    let expected = 12;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::num_of_ways(2);
    let expected = 54;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::num_of_ways(3);
    let expected = 246;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::num_of_ways(5000);
    let expected = 30228214;
    assert_eq!(actual, expected);
}
