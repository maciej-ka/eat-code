// https://leetcode.com/problems/final-value-of-variable-after-performing-operations/submissions/1806511912/

struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0;
        for op in operations {
            if op[1..2] == String::from("+") {
                result += 1;
            } else {
                result -= 1;
            }
        }
        result
    }
}

#[test]
fn test_1() {
    let actual = Solution::final_value_after_operations(["--X","X++","X++"].map(String::from).to_vec());
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::final_value_after_operations(["++X","++X","X++"].map(String::from).to_vec());
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::final_value_after_operations(["X++","++X","--X","X--"].map(String::from).to_vec());
    let expected = 0;
    assert_eq!(actual, expected);
}
