// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-ii/submissions/1944133469/?envType=daily-question&envId=2026-03-10

struct Solution;

const MOD: i64 = 1_000_000_007;

fn sum([a, b]: [i32; 2]) -> i32 {
    ((a as i64 + b as i64) % MOD) as i32
}

fn sub(a: i32, b: i32) -> i32 {
    let a = a as i64;
    let b = b as i64;
    (a - b).rem_euclid(MOD) as i32
}

impl Solution {

    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let one = one as usize;
        let zero = zero as usize;
        let limit = limit as usize;

        // dp[ones][zeros][zero turn, one turn]
        let mut dp = vec![vec![[0,0]; zero + 1]; one + 1];
        for i in 1 ..= one.min(limit) { dp[i][0] = [0,1] }
        for i in 1 ..= zero.min(limit) { dp[0][i] = [1,0] }

        for i in 1 ..= one {
            for k in 1 ..= zero {
                dp[i][k] = [sum(dp[i][k - 1]), sum(dp[i - 1][k])];
                if i > limit { dp[i][k][1] = sub(dp[i][k][1], dp[i - limit - 1][k][0]) }
                if k > limit { dp[i][k][0] = sub(dp[i][k][0], dp[i][k - limit - 1][1]) }
            }
        }

        sum(dp[one][zero])
    }
}

#[test]
fn test_1() {
    let actual = Solution::number_of_stable_arrays(1, 1, 2);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::number_of_stable_arrays(1, 2, 1);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::number_of_stable_arrays(3, 3, 2);
    let expected = 14;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::number_of_stable_arrays(15, 23, 29);
    let expected = 471286455;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::number_of_stable_arrays(1, 4, 2);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_6() {
    let actual = Solution::number_of_stable_arrays(24, 35, 21);
    let expected = 276183668;
    assert_eq!(actual, expected);
}

#[test]
fn test_7() {
    let actual = Solution::number_of_stable_arrays(425, 581, 287);
    let expected = 274434104;
    assert_eq!(actual, expected);
}
