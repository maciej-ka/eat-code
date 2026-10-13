// correct, but too slow

struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        Solution::solve(&target[..], 0)
    }

    fn solve(nums: &[i32], from: i32) -> i32 {
        let min = nums.iter().min().unwrap().clone();
        let mut res = min - from;

        // detect slices of non zeros after decrement
        let len = nums.len();
        let mut lo = 0;
        while lo < len {
            if nums[lo] <= min {
                lo += 1;
                continue;
            }

            let mut hi = lo;
            while hi < len && nums[hi] > min { hi += 1 }

            res += Solution::solve(&nums[lo..hi], min);
            lo = hi;
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_number_operations(vec![1, 2, 3, 2, 1]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_number_operations(vec![3, 1, 1, 2]);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_number_operations(vec![3, 1, 5, 4, 2]);
    let expected = 7;
    assert_eq!(actual, expected);
}
