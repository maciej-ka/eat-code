// https://leetcode.com/problems/find-unique-binary-string/submissions/1941929477/?envType=daily-question&envId=2026-03-08

struct Solution;

impl Solution {
    pub fn find_different_binary_string(mut nums: Vec<String>) -> String {
        let n = nums[0].len();
        nums.sort_unstable();

        let mut lo = 0;
        let mut hi = n;
        while lo < hi {
            let m = lo + hi >> 1;
            let val = usize::from_str_radix(&nums[m], 2).unwrap();
            if val == m { lo = m + 1 } else { hi = m }
        }

        format!("{:0>n$b}", lo, n = nums[0].len())
    }
}

#[test]
fn test_1() {
    let actual = Solution::find_different_binary_string(vec![String::from("01"), String::from("10")]);
    let expected = "00";
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::find_different_binary_string(vec![String::from("00"), String::from("01")]);
    let expected = "10";
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::find_different_binary_string(vec![String::from("111"), String::from("011"), String::from("001")]);
    let expected = "000";
    assert_eq!(actual, expected);
}
