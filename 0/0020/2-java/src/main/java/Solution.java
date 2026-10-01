// https://leetcode.com/problems/valid-parentheses/submissions/2158919560/?envType=daily-question&envId=2026-10-01
import java.util.Stack;

class Solution {
    public boolean isValid(String s) {
        Stack<Character> stack = new Stack<>();
        for (char symbol: s.toCharArray()) {
            if (symbol == ')') {
                if (stack.isEmpty() || stack.pop() != '(') return false;
            } else if (symbol == ']') {
                if (stack.isEmpty() || stack.pop() != '[') return false;
            } else if (symbol == '}') {
                if (stack.isEmpty() || stack.pop() != '{') return false;
            } else  {
                stack.push(symbol);
            }
        }
        return stack.isEmpty();
    }
}
