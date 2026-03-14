// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/submissions/1948090795/?envType=daily-question&envId=2026-03-14

struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let k = k as usize;
        let mut s = StrList::new();
        s.generate(n - 1, String::from("a"));
        s.generate(n - 1, String::from("b"));
        s.generate(n - 1, String::from("c"));
        if k > s.vec.len() { return String::from("") }
        s.vec[k - 1].clone()
    }
}

struct StrList {
    vec: Vec<String>
}

impl StrList {
    pub fn new() -> Self {
        StrList {
            vec: vec![]
        }
    }

    pub fn generate(&mut self, n: i32, prefix: String) {
        let last = prefix.chars().last().unwrap();
        if n == 0 {
            self.vec.push(prefix);
            return;
        }
        if last != 'a' { self.generate(n - 1, prefix.clone() + "a"); }
        if last != 'b' { self.generate(n - 1, prefix.clone() + "b"); }
        if last != 'c' { self.generate(n - 1, prefix.clone() + "c"); }
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
