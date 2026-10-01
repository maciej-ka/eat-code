// https://leetcode.com/problems/valid-parentheses/submissions/2159013828/?envType=daily-question&envId=2026-10-01
import java.util.Stack

class Solution {
    fun isValid(s: String): Boolean {
        val stack = Stack<Char>()
        for (char in s) {
            when (char) {
                ')' -> if (stack.isEmpty() || stack.pop() != '(') return false
                ']' -> if (stack.isEmpty() || stack.pop() != '[') return false
                '}' -> if (stack.isEmpty() || stack.pop() != '{') return false
                else -> stack.push(char)
            }
        }
        return stack.isEmpty()
    }
}
