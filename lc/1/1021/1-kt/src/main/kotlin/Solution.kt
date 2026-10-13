// https://leetcode.com/problems/remove-outermost-parentheses/submissions/2166072686/?envType=daily-question&envId=2026-10-08
import java.util.*

class Solution {
    fun removeOuterParentheses(s: String): String {
        var level = 0
        return buildString {
            for (c in s) {
                if (c == '(') {
                    if (level > 0) append(c)
                    level++
                } else {
                    level--
                    if (level > 0) append(c)
                }
            }
        }
    }
}
