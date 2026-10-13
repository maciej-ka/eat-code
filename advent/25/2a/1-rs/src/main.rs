use std::fs;

struct Solution;

impl Solution {
    fn solve(str: String) -> i64 {
        let mut ans = 0i64;
        for range in str.split(',') {
            let split: Vec<&str> = range.split('-').collect();
            let lo: i64 = split[0].trim().parse().unwrap();
            let hi: i64 = split[1].trim().parse().unwrap();
            for num in lo..=hi {
                let s = num.to_string();
                if s.len() & 1 == 1 { continue }
                let half = s.len() >> 1;
                if &s[0..half] == &s[half..] {
                    ans += num;
                }
            }
        }
        ans
    }
}

fn main() {
    let str = fs::read_to_string("./file.txt").unwrap();
    let res = Solution::solve(str);
    println!("password is: {:?}", res);
}

#[test]
fn test_1() {
    let str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let actual = Solution::solve(String::from(str));
    let expected = 1227775554;
    assert_eq!(actual, expected);
}
