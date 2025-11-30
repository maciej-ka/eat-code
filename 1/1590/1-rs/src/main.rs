// memory exceded

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as usize;
        let mut pos = vec![vec![]; p];
        let mut m = 0;

        for i in 0..nums.len() {
            m = (m + nums[i] as usize) % p;
            pos[m].push(i);
        }

        // all elements divide
        if m == 0 { return 0 }

        m = 0;
        let mut best = nums.len();

        for i in (0..nums.len()).rev() {
            let need = (p - m) % p;
            while let Some(k) = pos[need].last() {
                if *k >= i { pos[need].pop(); }
                else { break }
            }
            if let Some(k) = pos[need].last() {
                best = best.min(i - *k);
            }
            if need == 0 {
                best = best.min(i + 1);
            }
            m = (m + nums[i] as usize) % p;
        }

        if best == nums.len() { -1 } else { best as i32 }
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_subarray(vec![3, 1, 4, 2], 6);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_subarray(vec![6, 3, 5, 2], 9);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_subarray(vec![1, 2, 3], 3);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::min_subarray(vec![8, 32, 31, 18, 34, 20, 21, 13, 1, 27, 23, 22, 11, 15, 30, 4, 2], 148);
    let expected = 7;
    assert_eq!(actual, expected);
}
