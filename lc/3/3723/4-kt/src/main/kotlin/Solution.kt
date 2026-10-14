// https://leetcode.com/problems/maximize-sum-of-squares-of-digits/submissions/2171945969/?envType=daily-question&envId=2026-10-14
import java.util.*

class Solution {
    fun maxSumOfSquares(num: Int, sum: Int): String {
        if (sum > num * 9) return ""
        var rem = sum
        var chars = CharArray(num)
        var digits = charArrayOf('0', '1', '2', '3', '4', '5', '6', '7', '8', '9')
        for (i in 0 until num) {
            var digit = minOf(9, rem)
            rem -= digit
            chars[i] = digits[digit]
        }
        return String(chars)
    }
}
