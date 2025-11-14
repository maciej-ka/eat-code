// https://leetcode.com/problems/increment-submatrices-by-one/submissions/1829654804/

struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, mut queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mold = vec![0; n + 1];
        let mut leave: Vec<Vec<&Vec<i32>>> = vec![vec![]; n + 1];
        let mut res = vec![];
        let mut q = 0;
        let qlen = queries.len();
        queries.sort();

        for irow in 0..n {
            // incomming queries
            while q < qlen {
                let query = &queries[q];
                if query[0] as usize != irow { break }
                mold[query[1] as usize] += 1;
                mold[query[3] as usize + 1] -= 1;
                leave[query[2] as usize + 1].push(query);
                q += 1;
            }
            // leaving queries
            for query in &leave[irow] {
                mold[query[1] as usize] -= 1;
                mold[query[3] as usize + 1] += 1;
            }

            let mut sum = 0;
            let mut row: Vec<i32> = Vec::with_capacity(n);
            for icol in 0..n {
                sum += mold[icol];
                row.push(sum as i32);
            }
            res.push(row);
        }

        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]);
    let expected = vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::range_add_queries(2, vec![vec![0, 0, 1, 1]]);
    let expected = vec![vec![1, 1], vec![1, 1]];
    assert_eq!(actual, expected);
}
