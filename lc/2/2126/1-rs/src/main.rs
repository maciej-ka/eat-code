// https://leetcode.com/problems/destroying-asteroids/?envType=daily-question&envId=2026-05-31

struct Solution;

impl Solution {
    fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        asteroids.sort();
        for a in asteroids {
            let a = a as i64;
            if a > mass { return false }
            mass += a;
        }
        true
    }
}

#[test]
fn test_1() {
    let actual = Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]);
    let expected = false;
    assert_eq!(actual, expected);
}
