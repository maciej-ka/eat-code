// https://leetcode.com/problems/create-grid-with-exactly-one-path/submissions/2183340725/?envType=daily-question&envId=2026-10-27
import java.util.*

class Solution {
    fun createGrid(m: Int, n: Int): Array<String> {
        val topBuilder = StringBuilder()
        for (i in 0 until n) topBuilder.append('.')

        val rowBuilder = StringBuilder()
        for (i in 0 until n - 1) rowBuilder.append('#')
        rowBuilder.append('.')
        val row = rowBuilder.toString()

        var result = Array(m) { row }
        result[0] = topBuilder.toString()
        return result
    }
}
