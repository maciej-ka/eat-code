// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-ii/submissions/2020980472/?envType=daily-question&envId=2026-06-03

struct Solution;

impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>) -> i32 {
        let mut land_earliest = i32::MAX;
        let mut water_earliest = i32::MAX;

        for i in 0..land_start_time.len() {
            land_earliest = land_earliest.min(land_start_time[i] + land_duration[i]);
        }
        for i in 0..water_start_time.len() {
            water_earliest = water_earliest.min(water_start_time[i] + water_duration[i]);
        }

        let mut best = i32::MAX;
        for i in 0..land_start_time.len() {
            let start = water_earliest.max(land_start_time[i]);
            best = best.min(start + land_duration[i]);
        }
        for i in 0..water_start_time.len() {
            let start = land_earliest.max(water_start_time[i]);
            best = best.min(start + water_duration[i]);
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
