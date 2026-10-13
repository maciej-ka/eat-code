// https://leetcode.com/problems/maximum-running-time-of-n-computers/submissions/1844206132/

struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        let n = n as usize;
        batteries.sort_by(|a, b| b.cmp(a));

        let mut have = 0i64;
        for bat in &batteries[n..] {
            have += *bat as i64;
        }

        fn is_possible(goal: i64, batteries: &[i32], have: i64) -> bool {
            let mut need = 0;
            for bat in batteries {
                need += 0.max(goal - *bat as i64);
                if need > have { return false }
            }
            true
        }

        let mut lo = batteries[n - 1] as i64;
        let mut hi = batteries[0] as i64 + 1 + have / n as i64;
        while lo < hi {
            let m = lo + hi + 1 >> 1;
            if is_possible(m, &batteries[0..n], have) {
                lo = m;
            } else {
                hi = m - 1;
            }
        }

        lo
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_run_time(2, vec![3, 3, 3]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_run_time(2, vec![1, 1, 1, 1]);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_run_time(3, vec![1, 2, 5, 4, 3]);
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::max_run_time(3, vec![10, 10, 3, 5]);
    let expected = 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::max_run_time(1, vec![53, 96]);
    let expected = 149;
    assert_eq!(actual, expected);
}
