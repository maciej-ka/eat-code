// not doable like this

use std::fmt::Debug;
use std::collections::BTreeSet;

struct Solution;

struct FenwickTree<T> {
    arr: Vec::<T>
}

impl<T: Debug + Clone> FenwickTree<T> {
    fn new(size: usize, init: T) -> Self {
        FenwickTree { arr: vec![init; size] }
    }

    fn debug(&self, len: usize) {
        let len = len.min(self.arr.len());
        println!("{:?}", &self.arr[..len]);
    }
}

const MAX: usize = 450000;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let res = vec![];
        let mut gaps = FenwickTree::new(MAX, 0);
        let mut blocks = BTreeSet::<i32>::new();

        for query in queries {
            let at = query[1];
            match query[0] {
                1 => {
                    blocks.insert(at);
                }
                _ => ()
            }
        }

        gaps.debug(10);
        res
    }
}

// #[test]
// fn test_1() {
//     let actual = Solution::get_results(vec![
//         vec![1,2],
//         vec![2,3,3],
//         vec![2,3,1],
//         vec![2,2,2]
//     ]);
//     let expected = vec![false, true, true];
//     assert_eq!(actual, expected);
// }

// #[test]
// fn test_2() {
//     let actual = Solution::get_results(vec![
//         vec![1,7],
//         vec![2,7,6],
//         vec![1,2],
//         vec![2,7,5],
//         vec![2,7,6]
//     ]);
//     let expected = vec![true, true, false];
//     assert_eq!(actual, expected);
// }

#[test]
fn test_3() {
    let actual = Solution::get_results(vec![
        vec![1,20],
        vec![1,10],
        vec![1,15],
        vec![1,2],
    ]);
    let expected = vec![];
    assert_eq!(actual, expected);
}
