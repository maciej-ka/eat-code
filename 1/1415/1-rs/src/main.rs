// a bit wrong approach

struct Solution;

impl Solution {
    fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut last = 4;
        let mut curr;
        while n > 0 {
            curr = n % 3;
            if curr == last { return false }
            last = curr;
            n /= 3;
        }
        true
    }

    fn to_str(n: i32) -> String {
        let mut n = n;
        let mut str = String::from("");
        while n > 0 {
            let char = match n % 3 {
                2 => 'c',
                1 => 'b',
                _ => 'a',
            };
            str.insert(0, char);
            n /= 3;
        }
        str
    }

    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut num = 0;
        for i in 0..n {
            num *= 3;
            if i % 2 == 0 { num += 1 }
        }

        let max = 3i32.pow(3);
        let mut count = 0;
        while num < max {
            println!("");
            println!("num {:?}", num);
            println!("is_happy {:?}", Self::is_happy(num));
            println!("to_str {:?}", Self::to_str(num));

            if Self::is_happy(num) { count += 1; }
            if count == k { return Self::to_str(num) }
            num += 1;
        }

        return String::from("")
    }
}

#[test]
fn test_1() {
    let actual = Solution::get_happy_string(1, 3);
    let expected = "c";
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::get_happy_string(1, 4);
    let expected = "";
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::get_happy_string(3, 9);
    let expected = "cab";
    assert_eq!(actual, expected);
}
