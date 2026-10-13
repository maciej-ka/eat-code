// wrong

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        // index pointers
        let mut ind = Vec::with_capacity(intervals.len());
        for i in 0..intervals.len() { ind.push(i) }

        // trim ends
        // sort main by start
        // sort indices by end
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        ind.sort_by(|a, b| intervals[*a][1].cmp(&intervals[*b][1]));

        let mut val = -1;
        let mut i = 0;
        let mut k = 0;
        while i < ind.len() {
            if intervals[ind[i]][1] > val {
                val = intervals[ind[i]][1];
                while k < ind.len() && intervals[k][0] < val {
                    if intervals[k][1] > val { intervals[k][1] = val; }
                    k += 1;
                }
            }
            i += 1;
        }

        // trim starts
        // sort main by end desc
        // sort indices by start desc
        intervals.sort_by(|a, b| b[1].cmp(&a[1]));
        ind.sort_by(|a, b| intervals[*b][0].cmp(&intervals[*a][0]));

        val = intervals[ind[0]][0] + 1;
        i = 0;
        k = 0;
        while i < ind.len() {
            if intervals[ind[i]][0] < val {
                val = intervals[ind[i]][0];
                while k < ind.len() && intervals[k][1] > val {
                    if intervals[k][0] < val { intervals[k][0] = val; }
                    k += 1;
                }
            }
            i += 1;
        }

        let mut res = HashSet::new();
        for interval in intervals {
            res.insert(interval[0]);
            res.insert(interval[1]);
        }

        res.len() as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::intersection_size_two(vec![vec![1,3],vec![3,7],vec![8,9]]);
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::intersection_size_two(vec![vec![1,3],vec![1,4],vec![2,5],vec![3,5]],);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::intersection_size_two(vec![vec![1,2],vec![2,3],vec![2,4],vec![4,5]],);
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::intersection_size_two(vec![vec![4,6],vec![1,3],vec![2,8],vec![7,9]],);
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::intersection_size_two(vec![vec![1,3],vec![4,9],vec![0,10],vec![6,7],vec![1,2],vec![0,6],vec![7,9],vec![0,1],vec![2,5],vec![6,8]]); 
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn test_6() {
    let actual = Solution::intersection_size_two(vec![vec![2,15],vec![9,17],vec![0,6],vec![17,25],vec![0,25]]); 
    let expected = 5;
    assert_eq!(actual, expected);
}
