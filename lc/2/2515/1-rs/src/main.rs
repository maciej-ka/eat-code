// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/submissions/1979563085/?envType=daily-question&envId=2026-04-15

struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_index = start_index as usize;
        let n = words.len();

        for i in 0 ..= 1 + (n >> 1) {
            let i1 = (start_index + i) % n;
            let i2 = (n + start_index - i) % n;
            if words[i1] == target || words[i2] == target {
                return i as i32
            }
        }

        -1
    }
}

#[test]
fn test_1() {
    let actual = Solution::closest_target(vec![String::from("hello"), String::from("i"), String::from("am"), String::from("leetcode"), String::from("hello")], String::from("hello"), 1);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::closest_target(vec![String::from("a"), String::from("b"), String::from("leetcode")], String::from("leetcode"), 0);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::closest_target(vec![String::from("i"), String::from("eat"), String::from("leetcode")], String::from("ate"), 0);
    let expected = -1;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::closest_target(vec![String::from("a"), String::from("b"), String::from("leetcode")], String::from("leetcode"), 2);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::closest_target(
        vec![
            String::from("vieruszfqo"),
            String::from("ivgfhbopfy"),
            String::from("vfsymkzuio"),
            String::from("ryqtkehkov"),
            String::from("vfsymkzuio"),
            String::from("vieruszfqo"),
            String::from("xnsacsqaad"),
            String::from("xnsacsqaad"),
            String::from("ryqtkehkov"),
            String::from("oxnuqvvyqx")
        ],
        String::from("ryqtkehkov"),
        4);
    let expected = 1;
    assert_eq!(actual, expected);
}

