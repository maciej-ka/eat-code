// https://leetcode.com/problems/longest-common-suffix-queries/submissions/2015883157/?envType=daily-question&envId=2026-05-28

struct Solution;

const A: u8 = 'a' as u8;

#[derive(Debug)]
struct Node {
    i: usize,
    len: usize,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    pub fn new(i: usize, len: usize) -> Self {
        Self {
            i: i,
            len: len,
            children: Default::default(),
        }
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut root = Node::new(0, usize::MAX);

        for i in 0..words_container.len() {
            let word = words_container.get(i).unwrap();
            if root.len > word.len() {
                root.len = word.len();
                root.i = i;
            }

            let mut node = &mut root;
            for char in word.bytes().rev() {
                let k = (char - A) as usize;
                if node.children[k].is_none() {
                    node.children[k] = Some(Box::new(Node::new(i, word.len())))
                }
                node = node.children[k].as_mut().unwrap();
                if node.len > word.len() {
                    node.len = word.len();
                    node.i = i;
                }
            }
        }

        let mut res = vec![];

        for word in words_query {
            let mut node = &mut root;
            for char in word.bytes().rev() {
                let k = (char - A) as usize;
                if node.children[k].is_none() { break }
                node = node.children[k].as_mut().unwrap();
            }
            res.push(node.i as i32);
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::string_indices(
        vec![String::from("abcd"), String::from("bcd"), String::from("xbcd")],
        vec![String::from("cd"), String::from("bcd"), String::from("xyz")],
    );
    let expected = vec![1, 1, 1];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::string_indices(
        vec![String::from("abcdefgh"), String::from("poiuygh"), String::from("ghghgh")],
        vec![String::from("gh"), String::from("acbfgh"), String::from("acbfegh")],
    );
    let expected = vec![2, 0, 2];
    assert_eq!(actual, expected);
}
