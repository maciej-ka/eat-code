// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/submissions/1890439559/?envType=daily-question&envId=2026-01-19

class Solution {
    func maxSideLength(_ mat: [[Int]], _ threshold: Int) -> Int {
        let ymax = mat.count - 1
        let xmax = mat[0].count - 1
        var x = 0
        var y = 0
        var len = 0
        var best = 0
        var sum = mat[0][0]
        var dir = 1

        while true {
            // grow
            while x + len < xmax && sum <= threshold {
                len += 1
                best = len
                if y + len > ymax { return best }
                for i in 0...len {
                    sum += mat[y + len][x + i]
                    sum += mat[y + i][x + len]
                }
                sum -= mat[y + len][x + len]
            }

            if sum <= threshold {
                best = len + 1
            }

            if len == 0 && y == ymax && ((dir == 1 && x == xmax) || (dir == -1 && x == 0)) {
                return best
            }

            // move
            if 0 <= x + dir && x + len + dir <= xmax {
                x += dir
                if dir > 0 {
                    for i in 0...len {
                        sum += mat[y + i][x + len]
                        sum -= mat[y + i][x - 1]
                    }
                } else {
                    for i in 0...len {
                        sum += mat[y + i][x]
                        sum -= mat[y + i][x + len + 1]
                    }
                }
            } else {
                if len > 0 && y + len == ymax { break }
                y += 1
                dir *= -1
                for i in 0...len {
                    sum += mat[y + len][x + i]
                    sum -= mat[y - 1][x + i]
                }
            }
        }

        return best
    }
}
