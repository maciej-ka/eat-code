use std::fs;

struct Solution;

impl Solution {
    fn solve(str: String) -> usize {
        let mut res = 0;
        let mut dial = 50;

        for line in str.lines() {
            let sign = if &line[..=0] == "L" { -1 } else { 1 };
            let value = line[1..].parse::<i32>().unwrap() % 100;
            dial = (dial + 100 + value * sign) % 100;
            if dial == 0 { res += 1 }
        }

        res
    }
}

fn main() {
    let str = fs::read_to_string("./file.txt").unwrap();
    let res = Solution::solve(str);
    println!("password is: {:?}", res);
}

#[test]
fn test_1() {
    let str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let actual = Solution::solve(String::from(str));
    let expected = 3;
    assert_eq!(actual, expected);
}
