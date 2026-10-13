// https://leetcode.com/problems/valid-parentheses/submissions/2158995983/?envType=daily-question&envId=2026-10-01
import java.util.Stack

class Solution {
    fun isValid(s: String): Boolean {
        val stack = Stack<Char>()
        for (char in s) {
            if (char == ')') {
                if (stack.empty() || stack.pop() != '(') return false
            } else if (char == ']') {
                if (stack.empty() || stack.pop() != '[') return false
            } else if (char == '}') {
                if (stack.empty() || stack.pop() != '{') return false
            } else {
                stack.push(char)
            }
        }
        return stack.empty()
    }
}
