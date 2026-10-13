// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/submissions/1818558983/

struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;

        // board[Y][X]
        let mut board = vec![vec![0; n]; m];
        let mut res = m * n - guards.len() - walls.len();

        const X: usize = 1;
        const Y: usize = 0;
        const HOR: usize = 1;
        const VER: usize = 2;
        const OBS: usize = 3;

        for obs in &guards { board[obs[Y] as usize][obs[X] as usize] = OBS }
        for obs in &walls { board[obs[Y] as usize][obs[X] as usize] = OBS }

        let mut x: usize;
        let mut y: usize;

        // trace right
        for g in &guards {
            x = g[X] as usize + 1;
            y = g[Y] as usize;
            while x < n && board[y][x] == 0 {
                res -= 1;
                board[y][x] = HOR;
                x += 1;
            }
        }

        // trace left
        for g in &guards {
            if g[X] == 0 { continue }
            x = g[X] as usize - 1;
            y = g[Y] as usize;
            while board[y][x] == 0 {
                res -= 1;
                board[y][x] = HOR;
                if x == 0 { break }
                x -= 1;
            }
        }

        // trace down
        for g in &guards {
            x = g[X] as usize;
            y = g[Y] as usize + 1;
            while y < m && board[y][x] < VER {
                if board[y][x] == 0 { res -= 1 }
                board[y][x] = VER;
                y += 1;
            }
        }

        // trace up
        for g in &guards {
            if g[Y] == 0 { continue }
            x = g[X] as usize;
            y = g[Y] as usize - 1;
            while board[y][x] < VER {
                if board[y][x] == 0 { res -= 1 }
                board[y][x] = VER;
                if y == 0 { break }
                y -= 1;
            }
        }

        res as i32
    }
}

#[test]
fn test_1() {
    let actual = Solution::count_unguarded(
        4,
        6,
        vec![vec![0, 0], vec![1, 1], vec![2, 3]],
        vec![vec![0, 1], vec![2, 2], vec![1, 4]],
    );
    let expected = 7;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::count_unguarded(
        3,
        3,
        vec![vec![1, 1]],
        vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]],
    );
    let expected = 4;
    assert_eq!(actual, expected);
}
