// https://leetcode.com/problems/number-of-closed-islands/submissions/2181516136/?envType=daily-question&envId=2026-10-25
import java.util.*

class Solution {
    var rows = 0
    var cols = 0
    var grid = arrayOf(intArrayOf())

    fun closedIsland(grid: Array<IntArray>): Int {
        rows = grid.size
        cols = grid[0].size
        this.grid = grid

        var result = 0
        for (i in 0 until rows)
            for (k in 0 until cols)
                if (grid[i][k] == 0 && isPartOfSurroundedIsland(i, k))
                    result++
        return result
    }

    fun isPartOfSurroundedIsland(y: Int, x: Int): Boolean {
        grid[y][x] = 2
        val right = checkCell(y, x + 1)
        val down = checkCell(y + 1, x)
        val left = checkCell(y, x - 1)
        val up = checkCell(y - 1, x)
        return right && down && left && up
    }

    fun checkCell(y: Int, x: Int): Boolean {
        return when {
            x !in 0..<cols -> false
            y !in 0..<rows -> false
            grid[y][x] > 0 -> true
            else -> isPartOfSurroundedIsland(y, x)
        }
    }
}
