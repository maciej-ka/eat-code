// too slow
struct Solution;

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut res = 0;

        let mut sorted = nums.clone();
        sorted.sort_unstable();

        for k in 1..=len {
            if len % k != 0 { continue }
            let mut i = 0;

            while i < len {
                let twice = format!("{:?}", &sorted[i..i+k].repeat(2));
                let group = format!("{:?}", &nums[i..i+k]);
                if !&twice[1..twice.len()-1].contains(&group[1..group.len()-1]) {
                    break;
                }
                i += k;
            }
            if i == len { res += k }
        }

        res as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::sortable_integers(vec![3, 1, 2]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::sortable_integers(vec![7, 6, 5]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::sortable_integers(vec![5, 8]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::sortable_integers(vec![2, 3, 4, 5, 6, 1]);
    let expected = 6;
    assert_eq!(actual, expected);
}

