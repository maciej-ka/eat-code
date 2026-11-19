// https://leetcode.com/problems/minimum-score-by-changing-two-elements/submissions/2199604615/?envType=daily-question&envId=2026-11-19
struct Solution;

impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 { return 0; }

        let mut top = Top((i32::MIN, i32::MIN, i32::MIN));
        let mut bottom = Top((i32::MAX, i32::MAX, i32::MAX));

        for num in nums {
            top.high(num);
            bottom.low(num);
        }

        [
            top.0.0 - bottom.0.0,
            top.0.0 - bottom.0.1,
            top.0.0 - bottom.0.2,
            top.0.1 - bottom.0.0,
            top.0.1 - bottom.0.1,
            top.0.2 - bottom.0.0,
        ].iter().map(|x| x.abs()).min().unwrap()
    }
}

struct Top((i32, i32, i32));

impl Top {
    fn set2(&mut self, num: i32) {
        self.0.2 = num;
    }

    fn set1(&mut self, num: i32) {
        self.0.2 = self.0.1;
        self.0.1 = num;
    }

    fn set0(&mut self, num: i32) {
        self.0.2 = self.0.1;
        self.0.1 = self.0.0;
        self.0.0 = num;
    }

    fn high(&mut self, num: i32) {
        if num <= self.0.2 {
            return;
        }
        if num >= self.0.0 {
            self.set0(num);
        } else if num >= self.0.1 {
            self.set1(num);
        } else {
            self.set2(num);
        }
    }

    fn low(&mut self, num: i32) {
        if num >= self.0.2 {
            return
        }
        if num <= self.0.0 {
            self.set0(num);
        } else if num <= self.0.1 {
            self.set1(num);
        } else {
            self.set2(num);
        }
    }
}


#[test]
fn test_1() {
    let actual = Solution::minimize_sum(vec![1, 4, 7, 8, 5]);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::minimize_sum(vec![1, 4, 3]);
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::minimize_sum(vec![21,13,21,72,35,52,74]);
    let expected = 39;
    assert_eq!(actual, expected);
}
