// https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount/submissions/2018960301/?envType=daily-question&envId=2026-06-01

struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        let mut res = 0;
        for i in 1..=cost.len() {
            if i % 3 == 0 { continue }
            res += cost[cost.len() - i]
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::minimum_cost(vec![1, 2, 3]);
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]);
    let expected = 23;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::minimum_cost(vec![5, 5]);
    let expected = 10;
    assert_eq!(actual, expected);
}
