// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/submissions/2164006982/?envType=daily-question&envId=2026-10-06
import java.util.*

class Solution {
    fun minAddToMakeValid(s: String): Int {
        var adds = 0
        var opened = 0

        for (c in s) {
            when {
                c == '(' -> opened++
                opened != 0 -> opened--
                else -> adds++
            }
        }

        return adds + opened
    }
}
