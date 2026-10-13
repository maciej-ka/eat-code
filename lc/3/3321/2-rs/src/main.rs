// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii/submissions/1821909633/

struct Solution {}

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

const IN_MIN: bool = true;
const IN_MAX: bool = false;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let k = k as usize;
        let x = x as usize;

        let mut counts = HashMap::new();
        for n in &nums[0..k] {
            counts.entry(n).and_modify(|(count, _, _)| *count += 1).or_insert((1 as i64, 1, IN_MAX));
        }

        let mut init_list = Vec::with_capacity(counts.len());
        for (num, (count, _, _)) in &counts { init_list.push((*count, *num)) }
        init_list.sort();

        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();
        let mut sum: i64 = 0;

        let len = init_list.len();
        let mut space = (x as isize - len as isize).max(0);
        let imin = len - x.min(len);
        for (count, num) in &init_list[0..imin] { max_heap.push((*count, *num, 1)); }
        for (count, num) in &init_list[imin..len] {
            sum += *count * **num as i64;
            min_heap.push(Reverse((*count, *num, 1)));
            counts.entry(num).and_modify(|(_, _, side)| *side = IN_MIN);
        }

        let mut res = Vec::with_capacity(nums.len() - k + 1);
        res.push(sum);

        for i in 0..(nums.len() - k) {
            // leave
            let num = &nums[i];
            let (count, ver, side) = counts.get(&num).unwrap();
            if *side == IN_MIN {
                min_heap.push(Reverse((count - 1, num, ver + 1)));
                sum -= *num as i64;
            } else {
                max_heap.push((count - 1, num, ver + 1));
            }
            counts.entry(&num).and_modify(|(count, ver, _)| { *count -= 1; *ver += 1 });

            // arrive
            let num = &nums[i + k];
            if let None = counts.get(num) {
                if space > 0 {
                    counts.insert(num, (0 as i64, 0, IN_MIN));
                    space -= 1;
                } else {
                    counts.insert(num, (0 as i64, 0, IN_MAX));
                }
            }
            // counts.entry(num).or_insert((0 as i64, 0, IN_MAX));
            let (count, ver, side) = counts.get(&num).unwrap();
            if *side == IN_MIN {
                min_heap.push(Reverse((count + 1, num, ver + 1)));
                sum += *num as i64;
            } else {
                max_heap.push((count + 1, num, ver + 1));
            }
            counts.entry(&num).and_modify(|(count, ver, _)| { *count += 1; *ver += 1 });

            // clean heaps of stale data
            loop {
                if max_heap.len() == 0 { break }
                let (_, num, ver) = max_heap.peek().unwrap();
                if counts.get(num).unwrap().1 == *ver { break }
                max_heap.pop();
            }
            loop {
                let Reverse((_, num, ver)) = min_heap.peek().unwrap();
                if counts.get(num).unwrap().1 == *ver { break }
                min_heap.pop();
            }

            if max_heap.len() != 0 {
                let max = *max_heap.peek().unwrap();
                let Reverse(min) = *min_heap.peek().unwrap();
                if max > min {
                    let min_key = min.1;
                    let max_key = max.1;
                    counts.entry(&min_key).and_modify(|(_, _, side)| *side = IN_MAX);
                    counts.entry(&max_key).and_modify(|(_, _, side)| *side = IN_MIN);

                    min_heap.pop();
                    sum -= min.0 as i64 * *min.1 as i64;
                    max_heap.push(min);

                    max_heap.pop();
                    min_heap.push(Reverse(max));
                    sum += max.0 as i64 * *max.1 as i64;
                }
            }

            res.push(sum);
        }

        res
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

#[test]
fn test_4() {
    let actual = Solution::find_x_sum(vec![10,7,6,9,8], 2, 1);
    let expected = vec![10,7,9,9];
    assert_eq!(actual, expected);
}

#[test]
fn test_5() {
    let actual = Solution::find_x_sum(vec![2,5,3,5,1], 2, 1);
    let expected = vec![5,5,5,5];
    assert_eq!(actual, expected);
}

#[test]
fn test_6() {
    let actual = Solution::find_x_sum(vec![2,2,3,3,4,2], 3, 3);
    let expected = vec![7,8,10,9];
    assert_eq!(actual, expected);
}

#[test]
fn test_7() {
    let actual = Solution::find_x_sum(vec![5,4,6,5,4,4,5,3], 6, 5);
    let expected = vec![28,28,27];
    assert_eq!(actual, expected);
}

