// https://leetcode.com/problems/sum-of-sortable-integers/submissions/2185850835/?envType=daily-question&envId=2026-10-30
struct Solution;

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut res = 0;

        let mut sorted = nums.clone();
        sorted.sort_unstable();

        for k in 1..=len {
            if len % k != 0 { continue }
            let mut i = 0;
            while i < len {
                if !Self::includes(&sorted[i..i+k].repeat(2), &nums[i..i+k]) {
                    break;
                }
                i += k;
            }
            if i == len { res += k }
        }

        res as i32
    }

    fn includes(haystack: &[i32], needle: &[i32]) -> bool {
        let mut i = 0;
        let mut k = 0;
        while i + k < haystack.len() {
            if haystack[i + k] == needle[k] {
                k += 1;
                if k == needle.len() {
                    return true;
                }
            } else {
                i += 1;
                k = 0;
            }
        }
        false
    }
}

#[test]
fn test_1() {
    let actual = Solution::sortable_integers(vec![3, 1, 2]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::sortable_integers(vec![7, 6, 5]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::sortable_integers(vec![5, 8]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::sortable_integers(vec![2, 3, 4, 5, 6, 1]);
    let expected = 6;
    assert_eq!(actual, expected);
}

