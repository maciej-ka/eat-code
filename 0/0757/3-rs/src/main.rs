// wrong

struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort();

        let mut vec = Vec::with_capacity(intervals.len());
        let mut curr: Vec<i32>;
        let mut i = 0;
        while i < intervals.len() {
            curr = intervals[i].clone();

            // ignore all longer
            while i + 1 < intervals.len() && intervals[i + 1][0] == curr[0] {
                i += 1;
            }
            i += 1;

            // ignore current if has subset
            let mut k = i;
            let mut has_sub = false;
            while !has_sub && k < intervals.len() && intervals[k][0] < curr[1] {
                if intervals[k][1] <= curr[1] { has_sub = true }
                k += 1;
            }
            if has_sub { continue }

            // add current
            vec.push(curr);
        }
        println!("{:?}", vec);

        let mut res = 0;
        let mut touch = false;
        for i in 0..vec.len() - 1 {
            // can be moved to be inside next
            if vec[i][1] > vec[i + 1][0] {
                if touch {
                    res += 1;
                    touch = false;
                }
                vec[i + 1][1] = vec[i][1];
                continue;
            }

            if vec[i][1] >= vec[i + 1][0] {
                touch = true;
                res += 1;
                // vec[i + 1][0] = vec[i][1];
            } else {
                touch = false;
                res += 2;
            }
        }

        (res + 2) as i32
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

