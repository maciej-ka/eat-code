// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/submissions/1887267989/?envType=daily-question&envId=2026-01-16
struct Solution;

use std::collections::HashSet;

const MOD: i64 = 1000000007;

fn check(val: i32, best: &mut i32, widths: &HashSet<i32>) {
    if widths.contains(&val) && val > *best {
        *best = val;
    }
}

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let mut widths = HashSet::<i32>::new();
        widths.insert(n - 1);
        for i in 0..v_fences.len() {
            widths.insert(n - v_fences[i]);
            widths.insert(v_fences[i] - 1);
            for j in i + 1..v_fences.len() {
                widths.insert((v_fences[i] - v_fences[j]).abs());
            }
        }

        let mut best = -1;

        check(m - 1, &mut best, &widths);
        for i in 0..h_fences.len() {
            check(m - h_fences[i], &mut best, &widths);
            check(h_fences[i] - 1, &mut best, &widths);
            for j in i + 1..h_fences.len() {
                check((h_fences[i] - h_fences[j]).abs(), &mut best, &widths);
            }
        }

        if best > 0 {
            (best as i64 * best as i64 % MOD) as i32
        } else {
            -1
        }
    }
}

#[test]
fn test_1() {
    let actual = Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::maximize_square_area(6, 7, vec![2], vec![4]);
    let expected = -1;
    assert_eq!(actual, expected);
}
