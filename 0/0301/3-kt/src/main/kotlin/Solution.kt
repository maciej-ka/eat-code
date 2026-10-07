// https://leetcode.com/problems/remove-invalid-parentheses/submissions/2165792408/?envType=daily-question&envId=2026-10-07
import java.util.*

class Solution {

    val result = mutableListOf<String>()

    fun removeInvalidParentheses(s: String): List<String> {
        solve(s, 0, '(', ')')
        return result
    }

    fun solve(s: String, cutFrom: Int, open: Char, close: Char) {
        var opened = 0
        var i = -1
        while (++i < s.length)
            when {
                s[i] == open -> opened++
                s[i] != close -> {}
                opened > 0 -> opened--
                else -> break
            }

        if (i == s.length) {
            if (open == ')') result += s.reversed()
            else solve(s.reversed(), 0, ')', '(')
            return
        }

        var last = 'a'
        for (k in cutFrom..i) {
            if (s[k] == close && last != close)
                solve(s.substring(0, k) + s.substring(k + 1), k, open, close)
            last = s[k]
        }
    }
}
