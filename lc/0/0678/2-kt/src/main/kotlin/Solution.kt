// https://leetcode.com/problems/valid-parenthesis-string/submissions/2162518623/?envType=daily-question&envId=2026-10-04
import java.util.*
import kotlin.comparisons.maxOf

class Solution {
    fun checkValidString(s: String): Boolean {
        var min = 0
        var max = 0

        for (c in s) {
            when {
                c == '(' -> {
                    min++
                    max++
                }
                c == ')' -> {
                    if (max == 0) return false
                    min = maxOf(0, min - 1)
                    max--
                }
                else -> { // *
                    min = maxOf(0, min - 1)
                    max++
                }
            }
        }

        return min == 0
    }
}
