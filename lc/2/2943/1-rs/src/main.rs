// https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/?envType=daily-question&envId=2026-01-15

struct Solution;

impl Solution {
    fn max_seq_len(vec: &Vec<i32>) -> i32 {
        let mut last = vec[0];
        let mut len = 1;
        let mut best = 1;
        for &num in vec {
            if num == last + 1 {
                len += 1;
                best = best.max(len);
            } else {
                len = 1;
            }
            last = num;
        }
        best
    }

    pub fn maximize_square_hole_area(_n: i32, _m: i32, mut h_bars: Vec<i32>, mut v_bars: Vec<i32>) -> i32 {
        h_bars.sort();
        v_bars.sort();
        let max_h = Self::max_seq_len(&h_bars);
        let max_v = Self::max_seq_len(&v_bars);
        let min = max_h.min(max_v) + 1;
        min * min
    }
}

#[test]
fn test_1() {
    let actual = Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 4]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 3]);
    let expected = 9;
    assert_eq!(actual, expected);
}
