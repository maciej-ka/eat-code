// https://leetcode.com/problems/count-operations-to-obtain-zero/submissions/1824911550/

struct Solution;

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut num1 = num1;
        let mut num2 = num2;
        let mut res = 0;

        loop {
            if num2 > num1 {
                let temp = num1;
                num1 = num2;
                num2 = temp;
            }
            if num2 == 0 { break }
            num1 = num1 - num2;
            res += 1;
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::count_operations(2, 3);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::count_operations(10, 10);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::count_operations(79, 68);
    let expected = 1;
    assert_eq!(actual, expected);
}
