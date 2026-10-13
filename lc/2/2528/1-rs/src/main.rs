// https://leetcode.com/problems/maximize-the-minimum-powered-city/submissions/1823363494/

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let r = r as usize;
        let len = stations.len();
        let mut cities = Vec::with_capacity(len);
        let mut sum = 0i64;

        for i in 0..r { sum += stations[i] as i64 }
        for i in 0..len {
            let hi = i + r;
            if hi < len { sum += stations[hi] as i64 }
            if i > r { sum -= stations[i - r - 1] as i64 }
            cities.push(sum);
        }

        let mut lo = 0;
        let mut hi = 1e11 as i64;
        let mut m;

        while lo < hi {
            m = (lo + hi + 1) >> 1;
            if Self::is_possible(m, &cities, r, k) {
                lo = m;
            } else {
                hi = m - 1;
            }
        }

        lo
    }

    pub fn is_possible(res: i64, cities: &Vec<i64>, r: usize, k: i32) -> bool {
        let mut left = k as i64;
        let mut added = 0;
        let mut mods = VecDeque::new();

        for i in 0..cities.len() {
            if let Some((k, v)) = mods.front() {
                if i > *k {
                    added -= v;
                    mods.pop_front();
                }
            }

            let city = cities[i] as i64;
            if city + added < res {
                let needed: i64 = res - city - added;
                if needed > left { return false }
                left -= needed;
                added += needed;
                mods.push_back((i + (r << 1), needed));
            }
        }
        true
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_power(vec![1,2,4,5,0], 1, 2);
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_power(vec![4,4,4,4], 0, 3);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::is_possible(3, &vec![1,3,1], 1, 2);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::is_possible(3, &vec![1,3,1,1], 1, 2);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::is_possible(5, &vec![4,4,4,4], 0, 3);
    let expected = false;
    assert_eq!(actual, expected);
}
