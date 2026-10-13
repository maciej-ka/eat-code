// https://leetcode.com/problems/score-of-parentheses/submissions/2162958375/?envType=daily-question&envId=2026-10-05
import java.util.*

class Solution {
    fun scoreOfParentheses(s: String): Int {
        val stack = Stack<Int>()
        stack.push(0)

        for (c in s) {
            when {
                c == '(' -> stack.push(0)
                else -> {
                    val score = maxOf(1, stack.pop() * 2)
                    stack.push(stack.pop() + score)
                }
            }
        }

        return stack.pop()
    }
}
