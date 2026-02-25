// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/?envType=daily-question&envId=2026-02-25

struct Solution;

impl Solution {
    fn bits(num: &i32) -> i32 {
        let mut count = 0;
        let mut i = 1;
        while i <= *num {
            if i & num > 0 { count += 1; }
            i <<= 1;
        }
        count
    }

    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut vec = Vec::<(i32, i32)>::with_capacity(arr.len());
        for n in arr {
            vec.push((Self::bits(&n), n));
        }
        vec.sort_by(|a, b| {
            if a.0 == b.0 { (a.1).cmp(&b.1) }
            else { (a.0).cmp(&b.0) }
        });
        vec.iter().map(|(_, n)| *n).collect()
    }
}

#[test]
fn test_1() {
    let actual = Solution::sort_by_bits(vec![0,1,2,3,4,5,6,7,8]);
    let expected = [0,1,2,4,8,3,5,6,7];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::sort_by_bits(vec![1024,512,256,128,64,32,16,8,4,2,1]);
    let expected = [1,2,4,8,16,32,64,128,256,512,1024];
    assert_eq!(actual, expected);
}
