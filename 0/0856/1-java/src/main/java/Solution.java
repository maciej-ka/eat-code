// https://leetcode.com/problems/score-of-parentheses/submissions/2162931843/?envType=daily-question&envId=2026-10-05
import java.util.*;

class Solution {
    public int scoreOfParentheses(String s) {
        var stack = new Stack<Integer>();
        stack.push(0);

        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == '(') {
                stack.push(0);
            } else {
                var score = Math.max(1, stack.pop() * 2);
                stack.push(stack.pop() + score);
            }
        }

        return stack.pop();
    }
}
