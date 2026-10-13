// too slow

struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn combinations(n: usize, limit: usize, lenght: usize, counts: &mut Vec<i64>) -> i64 {
        if n == 0 { counts[lenght] += 1; }
        let mut ans = 0i64;
        for i in 1 ..= n.min(limit) {
            let follow = Self::combinations(n - i, limit, lenght + 1, counts);
            ans = (ans + follow) % MOD;
        }
        ans
    }

    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let limit = limit as usize;

        let zero = zero as usize;
        let mut count0 = vec![0; zero + 1 as usize];
        Self::combinations(zero, limit, 0, &mut count0);

        let one = one as usize;
        let mut count1 = vec![0; one + 1 as usize];
        Self::combinations(one, limit, 0, &mut count1);

        let mut ans = 0i64;
        for i in 1..(zero + 1) {
            for k in (i - 1)..=(i + 1) {
                if k >= one + 1 { continue }
                if k == i { ans = (ans + count0[i] * count1[k] * 2) % MOD }
                else { ans = (ans + count0[i] * count1[k]) % MOD }
            }
        }

        ans as i32
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
