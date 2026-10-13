// https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/submissions/1805127793/

struct Solution;

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut last = i32::MIN;
        let mut result = 0;

        for num in nums {
            if num - k > last {
                last = num - k;
                result += 1;
                continue;
            }

            if num + k > last {
                last += 1;
                result += 1;
            }
        }

        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_distinct_elements(vec![1,2,2,3,3,4], 2);
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_distinct_elements(vec![1,2,2,2,2,4,4,4,4,3], 2);
    let expected = 8;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_distinct_elements(vec![4,4,4,4], 1);
    let expected = 3;
    assert_eq!(actual, expected);
}
