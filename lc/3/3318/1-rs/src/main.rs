// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/submissions/1820644464/

struct Solution {}

const NONE: usize = 100;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let x = x as usize;
        let mut lookup = [NONE; 51];
        let mut ranking = Vec::<Count>::with_capacity(51);

        for i in 0..k {
            Solution::change(nums[i] as usize, 1, &mut ranking, &mut lookup);
        }

        let mut res = Vec::<i32>::with_capacity(nums.len() - k + 1);
        res.push(Solution::sum(&ranking, x));

        for i in 0..(nums.len() - k) {
            Solution::change(nums[i] as usize, -1, &mut ranking, &mut lookup);
            Solution::change(nums[i + k] as usize, 1, &mut ranking, &mut lookup);
            res.push(Solution::sum(&ranking, x));
        }

        res
    }

    fn sum(ranking: &Vec<Count>, x: usize) -> i32 {
        let mut res: i32 = 0;
        for i in 0..ranking.len().min(x) {
            let count = ranking[i].count as i32;
            if count == 0 { return res }
            res += count * ranking[i].num as i32;
        }
        res
    }

    fn swap(i: usize, j: usize, ranking: &mut Vec<Count>, lookup: &mut [usize; 51]) {
        lookup[ranking[i].num] = j;
        lookup[ranking[j].num] = i;

        let temp = ranking[i].clone();
        ranking[i] = ranking[j];
        ranking[j] = temp;
    }

    fn change(num: usize, delta: isize, ranking: &mut Vec<Count>, lookup: &mut [usize; 51]) {
        let mut i;

        if lookup[num] == NONE {
            ranking.push(Count { num, count: 1 });
            i = ranking.len() - 1;
            lookup[num] = i;
        } else {
            i = lookup[num];
            ranking[i].count += delta;
        }

        if delta > 0 {
            while i > 0
                && (ranking[i - 1].count < ranking[i].count
                || (ranking[i - 1].count == ranking[i].count && ranking[i - 1].num < ranking[i].num))
            {
                Solution::swap(i, i - 1, ranking, lookup);
                i -= 1;
            }
        } else {
            while i < ranking.len() - 1
                && (ranking[i + 1].count > ranking[i].count
                || (ranking[i + 1].count == ranking[i].count && ranking[i + 1].num > ranking[i].num))
            {
                Solution::swap(i, i + 1, ranking, lookup);
                i += 1;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Count {
    num: usize,
    count: isize,
}

#[test]
fn test_1() {
    let actual = Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2);
    let expected = vec![6, 10, 12];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2);
    let expected = vec![11, 15, 15, 15, 12];
    assert_eq!(actual, expected);
}
