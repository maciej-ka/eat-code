// https://leetcode.com/problems/create-grid-with-exactly-one-path/submissions/2183348400/?envType=daily-question&envId=2026-10-27
import java.util.*

class Solution {
    fun createGrid(m: Int, n: Int): Array<String> {
        val charArr = CharArray(n) { '#' }
        charArr[n - 1] = '.'
        val row = String(charArr)
        var result = Array(m) { row }
        result[0] = String(CharArray(n) { '.' })
        return result
    }
}
