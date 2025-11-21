// https://leetcode.com/problems/set-intersection-size-at-least-two/submissions/1835588185/

struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| {
            if a[1] == b[1] {
                a[0].cmp(&b[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        let mut res = 0;
        let mut n1 = -1;
        let mut n2 = -1;

        for inter in intervals {
            if inter[0] > n2 {
                res += 2;
                n1 = inter[1] - 1;
                n2 = inter[1];
            } else if inter[0] > n1 {
                res += 1;
                if n2 == inter[1] {
                    n1 = inter[1] - 1
                } else {
                    n1 = n2;
                    n2 = inter[1];
                }
            }
        }

        res
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

#[test]
fn test_7() {
    let actual = Solution::intersection_size_two(vec![vec![3,13],vec![2,8],vec![5,10]]); 
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_8() {
    let actual = Solution::intersection_size_two(vec![vec![33,44],vec![42,43],vec![13,37],vec![24,33],vec![24,33],vec![25,48],vec![10,47],vec![18,24],vec![29,37],vec![7,34]]); 
    let expected = 6;
    assert_eq!(actual, expected);
}

#[test]
fn test_9() {
    let actual = Solution::intersection_size_two(vec![vec![3,14],vec![4,14],vec![3,9],vec![5,13],vec![10,17],vec![8,20],vec![7,12],vec![15,19],vec![11,17],vec![6,18],vec![16,20],vec![2,18],vec![3,5],vec![15,18],vec![9,12],vec![3,14],vec![10,15],vec![1,13],vec![8,10],vec![0,20]]); 
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn test_10() {
    let actual = Solution::intersection_size_two(vec![vec![1,3],vec![3,7],vec![5,7],vec![7,8]]); 
    let expected = 5;
    assert_eq!(actual, expected);
}


