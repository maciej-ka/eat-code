// https://leetcode.com/problems/longest-valid-parentheses/submissions/2161190399/?envType=daily-question&envId=2026-10-03
import java.util.*

class Solution {
    fun longestValidParentheses(s: String): Int {
        var best = 0
        val stack = Stack<Int>();
        var i = 0
        while (i < s.length) {
            when {
                s[i] == '(' -> stack.push(i)
                stack.isEmpty() -> {}
                i+1 < s.length && s[i+1] == '(' -> {
                    best = maxOf(best, i - stack.peek() + 1)
                    i++
                }
                else ->
                    best = maxOf(best, i - stack.pop() + 1)
            }
            i++
        }
        return best
    }
}
