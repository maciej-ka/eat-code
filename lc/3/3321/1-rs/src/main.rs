// too slow

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let k = k as usize;
        let x = x as usize;
        let mut lookup: HashMap<usize, usize> = HashMap::new();
        let mut ranking = Vec::new();

        for i in 0..k {
            Solution::change(nums[i] as usize, 1, &mut ranking, &mut lookup);
        }

        let mut res = Vec::<i64>::with_capacity(nums.len() - k + 1);
        res.push(Solution::sum(&ranking, x));

        for i in 0..(nums.len() - k) {
            Solution::change(nums[i] as usize, -1, &mut ranking, &mut lookup);
            Solution::change(nums[i + k] as usize, 1, &mut ranking, &mut lookup);
            res.push(Solution::sum(&ranking, x));
        }

        res
    }

    fn sum(ranking: &Vec<(isize, usize)>, x: usize) -> i64 {
        let mut res: i64 = 0;
        for i in 0..ranking.len().min(x) {
            let count = ranking[i].0 as i64;
            if count == 0 { return res }
            res += count * ranking[i].1 as i64;
        }
        res
    }

    fn swap(i: usize, j: usize, ranking: &mut Vec<(isize, usize)>, lookup: &mut HashMap<usize, usize>) {
        lookup.insert(ranking[i].1, j);
        lookup.insert(ranking[j].1, i);

        let temp = ranking[i];
        ranking[i] = ranking[j];
        ranking[j] = temp;
    }

    fn change(num: usize, delta: isize, ranking: &mut Vec<(isize, usize)>, lookup: &mut HashMap<usize, usize>) {
        let mut i;

        match lookup.get(&num) {
            None => {
                ranking.push((1, num));
                i = ranking.len() - 1;
                lookup.insert(num, i);
            }
            Some(x) => {
                i = *x;
                ranking[i].0 += delta;
            }
        }

        // increased
        if delta > 0 {
            while i > 0 && ranking[i] > ranking[i - 1] {
                Solution::swap(i, i - 1, ranking, lookup);
                i -= 1;
            }
        // decreased
        } else {
            while i < ranking.len() - 1 && ranking[i] < ranking[i + 1] && ranking[i + 1].0 > 0 {
                Solution::swap(i, i + 1, ranking, lookup);
                i += 1;
            }
        }
    }
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

#[test]
fn test_3() {
    let actual = Solution::find_x_sum(vec![1000000000,1000000000,1000000000,1000000000,1000000000,1000000000], 6, 1);
    let expected = vec![6000000000];
    assert_eq!(actual, expected);
}
