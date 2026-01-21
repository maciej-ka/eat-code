struct Solution;

impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            nums[i] = if nums[i] & 1 == 0 {
                -1
            } else {
                let b = (nums[i] + 1) & -(nums[i] + 1);
                nums[i] - (b >> 1)
            }
        }
        nums
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_bitwise_array(vec![2,3,5,7]);
    let expected = vec![-1,1,4,3];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_bitwise_array(vec![11,13,31]);
    let expected = vec![9,12,15];
    assert_eq!(actual, expected);
}
