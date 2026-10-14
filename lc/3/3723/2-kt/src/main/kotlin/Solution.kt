// https://leetcode.com/problems/maximize-sum-of-squares-of-digits/submissions/2171928575/?envType=daily-question&envId=2026-10-14
import java.util.*
import kotlin.math.max

class Solution {
    fun maxSumOfSquares(num: Int, sum: Int): String {
        if (sum > num * 9) return ""
        return buildString {
            val nines = sum / 9
            append("9".repeat(nines))
            if (nines < num) append(sum % 9)
            val zeroes = num - nines - 1
            if (zeroes > 0) append("0".repeat(zeroes))
        }
    }
}
