use std::fs;

struct Solution;

impl Solution {
    fn solve(str: String) -> i32 {
        let mut res = 0;
        let mut dial = 50;

        for line in str.lines() {
            let sign = if &line[..=0] == "L" { -1 } else { 1 };
            let value = line[1..].parse::<i32>().unwrap();
            let old = dial;
            dial += value * sign;
            if dial == 0 {
                res += 1;
                continue;
            }
            if old > 0 && dial < 0 { res += 1 }
            res += (dial / 100).abs();
            dial = (100 + dial % 100) % 100;
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
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let str = "L50
L200
R1
L1";
    let actual = Solution::solve(String::from(str));
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let str = "L50
L200
R1
L1";
    let actual = Solution::solve(String::from(str));
    let expected = 4;
    assert_eq!(actual, expected);
}
