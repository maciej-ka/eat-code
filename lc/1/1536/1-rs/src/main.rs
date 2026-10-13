// https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/submissions/1935723549/?envType=daily-question&envId=2026-03-02

struct Solution;

impl Solution {
    fn tail_zeroes(arr: &Vec<i32>) -> i32 {
        let mut count = 0;
        for i in (0..arr.len()).rev() {
            if arr[i] != 0 { break }
            count += 1
        }
        count
    }

    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let mut vec: Vec::<i32> = grid.iter().map(Self::tail_zeroes).collect();

        let len = grid.len();
        let mut ans = 0;

        for i in 0..len {
            let goal = (len - i - 1) as i32;
            let mut j = i;

            // look for next
            while vec[j] < goal {
                j += 1;
                if j >= len { return -1 }
            }

            // bubble up
            while j > i {
                let tmp = vec[j];
                vec[j] = vec[j - 1];
                vec[j - 1] = tmp;
                ans += 1;
                j -= 1;
            }
        }
        ans
    }
}

#[test]
fn test_1() {
    let actual = Solution::min_swaps(vec![vec![0,0,1],vec![1,1,0],vec![1,0,0]]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::min_swaps(vec![vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0]]);
    let expected = -1;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::min_swaps(vec![vec![1,0,0],vec![1,1,0],vec![1,1,1]]);
    let expected = 0;
    assert_eq!(actual, expected);
}
