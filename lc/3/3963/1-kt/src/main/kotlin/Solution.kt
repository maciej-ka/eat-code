// https://leetcode.com/problems/create-grid-with-exactly-one-path/submissions/2183327916/?envType=daily-question&envId=2026-10-27
import java.util.*

class Solution {
    fun createGrid(m: Int, n: Int): Array<String> {
        var result = arrayOf(".".repeat(n))
        val row = "#".repeat(n - 1) + "."
        for (i in 0 until m-1) result += row
        return result
    }
}
