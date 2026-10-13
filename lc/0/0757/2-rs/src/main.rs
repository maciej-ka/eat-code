// wrong

struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort();
        println!("{:?}", intervals);
        let mut curr = &vec![-2, -1];
        let mut last: &Vec<i32> = &vec![-2, -1];
        let mut res = 0;
        let mut i = 0;

        while i < intervals.len() {
            last = curr;
            curr = &intervals[i];
            println!("");
            println!("{:?}", curr);

            // ignore all longer
            while i + 1 < intervals.len() && intervals[i + 1][0] == curr[0] {
                i += 1;
            }
            println!("i {:?}", i);

            // if last, add both
            if i == intervals.len() - 1 {
                println!("{:?}", "last");
                res += 2;
                break;
            }

            // set i on next start
            i += 1;

            // if there is a sub interval, skip current
            let mut k = i;
            let mut found = false;
            while k < intervals.len() && intervals[k][0] < curr[1] {
                if intervals[k][1] <= curr[1] {
                    found = true;
                    break;
                }
                k += 1;
            }
            println!("sub {:?}", found);
            if found {
                // if there was touch last time, increase by one
                if last[1] >= curr[0] && last[1] < intervals[k][0] { res += 1 }
                continue
            }

            // if there is touch add one, if not, add two
            res += if intervals[i][0] <= curr[1] { 1 } else { 2 };
            println!("res {:?}", res);
        }

        res as i32
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
