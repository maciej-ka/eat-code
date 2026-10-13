// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-i/submissions/2020207842/?envType=daily-question&envId=2026-06-02

struct Solution;

impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>) -> i32 {
        let mut best = i32::MAX;

        for i in 0..land_start_time.len() {
            for k in 0..water_start_time.len() {
                let mut time = land_start_time[i] + land_duration[i];
                time = time.max(water_start_time[k]) + water_duration[k];
                best = best.min(time);
            }
        }

        for i in 0..water_start_time.len() {
            for k in 0..land_start_time.len() {
                let mut time = water_start_time[i] + water_duration[i];
                time = time.max(land_start_time[k]) + land_duration[k];
                best = best.min(time);
            }
        }

        best
    }
}


#[test]
fn test_1() {
    let actual = Solution::earliest_finish_time(vec![2,8], vec![4,1], vec![6], vec![3]);
    let expected = 9;
    assert_eq!(actual, expected);
}


#[test]
fn test_2() {
    let actual = Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]);
    let expected = 14;
    assert_eq!(actual, expected);
}
