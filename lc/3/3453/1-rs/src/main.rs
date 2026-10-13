// https://leetcode.com/problems/separate-squares-i/submissions/1884054276/?envType=daily-question&envId=2026-01-13

struct Solution;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut events = Vec::<(i64, i64)>::with_capacity(squares.len() << 1);
        let mut total = 0i64;
        for s in squares {
            let y = s[1] as i64;
            let l = s[2] as i64;
            events.push((y, l));
            events.push((y + l, -l));
            total += l * l;
        }
        events.sort();

        let goal = total as f64 / 2.0;
        let mut h = 0i64;
        let mut active = 0i64;
        let mut sum = 0i64;

        for e in events {
            let dh = e.0 - h;
            let dsum = active * dh;
            if (sum + dsum) as f64 >= goal {
                let need = goal - sum as f64;
                return h as f64 + dh as f64 * need as f64 / dsum as f64;
            }
            active += e.1;
            sum += dsum;
            h = e.0;
        }

        return 0.0
    }
}

#[test]
fn test_1() {
    let actual = Solution::separate_squares(vec![vec![0,0,1], vec![2,2,1]]);
    let expected = 1.0;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::separate_squares(vec![vec![0,0,2], vec![1,1,1]]);
    let expected = 1.1666666666666667;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::separate_squares(vec![vec![0,0,8], vec![0,3,2], vec![0,4,6]]);
    let expected = 5.142857142857143;
    assert_eq!(actual, expected);
}
