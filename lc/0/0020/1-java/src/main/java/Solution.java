// https://leetcode.com/problems/valid-parentheses/submissions/2158899426/?envType=daily-question&envId=2026-10-01
import java.util.Stack;

class Solution {
    public boolean isValid(String s) {
        Stack<String> stack = new Stack<>();
        for (int i = 0; i < s.length(); i++) {
            char symbol = s.charAt(i);
            if (symbol == ')') {
                if (stack.isEmpty() || !stack.pop().equals("(")) return false;
            } else if (symbol == ']') {
                if (stack.isEmpty() || !stack.pop().equals("[")) return false;
            } else if (symbol == '}') {
                if (stack.isEmpty() || !stack.pop().equals("{")) return false;
            } else  {
                stack.push(symbol + "");
            }
        }
        return stack.size() == 0;
    }
}
