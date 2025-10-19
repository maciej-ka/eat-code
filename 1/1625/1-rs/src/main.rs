// https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/submissions/1806016386
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let a: u32 = a.try_into().unwrap();
        let b: usize = b.try_into().unwrap();
        let mut best = s.clone();
        let mut vec = Vec::new();
        for char in s.chars() {
            vec.push(char.to_digit(10).unwrap());
        }

        let mut visited = HashSet::new();
        visited.insert(vec.clone());

        let mut stack = Vec::new();
        stack.push(vec);

        while stack.len() > 0 {
            vec = stack.pop().unwrap();
            let str = Solution::to_string(vec.clone());
            if str < best { best = str }

            let vec_a = Solution::add(vec.clone(), a);
            if !visited.contains(&vec_a) {
                visited.insert(vec_a.clone());
                stack.push(vec_a);
            }

            let vec_b = Solution::rotate(vec.clone(), b);
            if !visited.contains(&vec_b) {
                visited.insert(vec_b.clone());
                stack.push(vec_b);
            }
        }

        best
    }

    pub fn add(mut vec: Vec<u32>, a: u32) -> Vec<u32> {
        for i in 0..vec.len() {
            if i % 2 == 1 { vec[i] = (vec[i] + a) % 10 }
        }
        vec
    }

    pub fn rotate(mut vec: Vec<u32>, b: usize) -> Vec<u32> {
        let drained: Vec<u32> = vec.drain(0..b).collect();
        vec.extend(drained);
        vec
    }

    pub fn to_string(vec: Vec<u32>) -> String {
        vec.iter().map(|n| n.to_string()).collect()
    }
}

#[test]
fn test_1() {
    let actual = Solution::find_lex_smallest_string(String::from("5525"), 9, 2);
    let expected = "2050";
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::find_lex_smallest_string(String::from("74"), 5, 1);
    let expected = "24";
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::find_lex_smallest_string(String::from("0011"), 4, 2);
    let expected = "0011";
    assert_eq!(actual, expected);
}
