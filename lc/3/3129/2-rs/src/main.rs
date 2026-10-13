// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-i/submissions/1943359945/?envType=daily-question&envId=2026-03-09

struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        if zero == 0 || one == 0 { return 0 }
        let zero = zero as usize;
        let one = one as usize;
        let mut s = MyStruct::new(zero, one, limit as usize);

        let mut ans = 0;
        if s.is_possible(0, zero, one) { ans = (ans + s.solve(0, zero, one)) % MOD }
        if s.is_possible(1, zero, one) { ans = (ans + s.solve(1, zero, one)) % MOD }
        ans as i32
    }
}

struct MyStruct {
    memo: Vec<Vec<Vec<i64>>>,
    limit: usize,
}

impl MyStruct {
    fn new(zeroes: usize, ones: usize, limit: usize) -> Self {
        let mut s = MyStruct {
            memo: vec![vec![vec![0i64; ones + 1]; zeroes + 1]; 2],
            limit
        };
        for i in 1..=zeroes.min(limit) { s.memo[0][i][0] = 1 }
        for i in 1..=ones.min(limit) { s.memo[1][0][i] = 1 }
        s
    }

    fn is_possible(&mut self, turn: usize, zeroes: usize, ones: usize) -> bool {
        let me = if turn == 0 { zeroes } else { ones };
        let other = if turn == 0 { ones } else { zeroes };
        if other == 0 && me <= self.limit { return true }
        if me > (other + 1) * self.limit { return false }
        if other > me * self.limit { return false }
        true
    }

    fn solve(&mut self, turn: usize, zeroes: usize, ones: usize) -> i64 {
        // at this point we checked that result is possible
        // so it should be at least 1
        let m = self.memo[turn][zeroes][ones];
        if m > 0 { return m }

        let mut ans = 0i64;
        if turn == 0 {
            for i in 1..=zeroes.min(self.limit) {
                if self.is_possible(1, zeroes - i, ones) {
                    ans = (ans + self.solve(1, zeroes - i, ones)) % MOD;
                }
            }
        } else {
            for i in 1..=ones.min(self.limit) {
                if self.is_possible(0, zeroes, ones - i) {
                    ans = (ans + self.solve(0, zeroes, ones - i)) % MOD;
                }
            }
        }

        self.memo[turn][zeroes][ones] = ans;
        ans
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
