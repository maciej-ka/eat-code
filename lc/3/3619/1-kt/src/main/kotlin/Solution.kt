// https://leetcode.com/problems/count-islands-with-total-value-divisible-by-k/submissions/2182177360/?envType=daily-question&envId=2026-10-26
import java.util.*

class Solution {
    var grid: Array<IntArray> = emptyArray()
    var rows = 0
    var cols = 0

    fun countIslands(grid: Array<IntArray>, k: Int): Int {
        this.grid = grid
        rows = grid.size
        cols = grid[0].size

        var result = 0
        for (y in 0 until rows)
            for (x in 0 until cols)
                if (grid[y][x] > 0 && getIslandSum(y, x, 0L) % k == 0L)
                    result++

        return result
    }

    fun getIslandSum(y: Int, x: Int, sum: Long): Long {
        if (y !in 0..<rows) return sum
        if (x !in 0..<cols) return sum
        if (grid[y][x] <= 0) return sum

        var result = sum + grid[y][x]
        grid[y][x] = -1

        result = getIslandSum(y + 1, x, result)
        result = getIslandSum(y - 1, x, result)
        result = getIslandSum(y, x + 1, result)
        result = getIslandSum(y, x - 1, result)
        return result
    }
}
