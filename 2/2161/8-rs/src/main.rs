// https://leetcode.com/problems/partition-array-according-to-given-pivot/submissions/2026465831/?envType=daily-question&envId=2026-06-08

struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(nums.len());
        let mut greater = Vec::<i32>::with_capacity(nums.len());
        let mut count_pivots = 0;

        for num in nums {
            if num > pivot { greater.push(num) }
            else if num == pivot { count_pivots += 1 }
            else { result.push(num); }
        }

        for _ in 0..count_pivots {
            result.push(pivot);
        }

        result.extend(greater);

        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::pivot_array(vec![9,12,5,10,14,3,10], 10);
    let expected = vec![9,5,3,10,10,12,14];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::pivot_array(vec![-3,4,3,2], 2);
    let expected = vec![-3,2,4,3];
    assert_eq!(actual, expected);
}
