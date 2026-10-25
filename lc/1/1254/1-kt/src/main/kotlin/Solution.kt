// https://leetcode.com/problems/number-of-closed-islands/submissions/2181504620/?envType=daily-question&envId=2026-10-25
import java.util.*

class Solution {
    var rows = 0
    var cols = 0
    var grid = arrayOf(intArrayOf())
    var visited = arrayOf(intArrayOf())

    fun closedIsland(grid: Array<IntArray>): Int {
        rows = grid.size
        cols = grid[0].size
        this.grid = grid
        this.visited = Array(rows) { IntArray(cols) { 0 } }

        var result = 0
        for (i in 0 until rows)
            for (k in 0 until cols)
                if (grid[i][k] == 0 && visited[i][k] == 0 && isPartOfSurroundedIsland(i, k))
                    result++
        return result
    }

    fun isPartOfSurroundedIsland(y: Int, x: Int): Boolean {
        visited[y][x] = 1
        val neighbours = arrayOf(
            intArrayOf(x + 1, y),
            intArrayOf(x, y + 1),
            intArrayOf(x - 1, y),
            intArrayOf(x, y - 1)
        )

        var result = true
        for ((xi, yi) in neighbours) {
            when {
                xi !in 0..<cols -> result = false
                yi !in 0..<rows -> result = false
                grid[yi][xi] == 1 -> continue
                visited[yi][xi] == 1 -> continue
                !isPartOfSurroundedIsland(yi, xi) -> result = false
            }
        }
        return result
    }
}
