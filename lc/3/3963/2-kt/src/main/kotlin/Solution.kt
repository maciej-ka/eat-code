// https://leetcode.com/problems/create-grid-with-exactly-one-path/submissions/2183333573/?envType=daily-question&envId=2026-10-27
import java.util.*

class Solution {
    fun createGrid(m: Int, n: Int): Array<String> {
        val top = buildString {
            for (i in 0 until n) append('.')
        }
        val row = buildString {
            for (i in 0 until n - 1) append('#')
            append('.')
        }

        var result = arrayOf(top)
        for (i in 0 until m-1) result += row
        return result
    }
}
