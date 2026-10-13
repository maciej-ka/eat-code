// https://leetcode.com/problems/longest-valid-parentheses/submissions/2161154538/?envType=daily-question&envId=2026-10-03
import java.util.*;

class Solution {
    public int longestValidParentheses(String s) {
        int best = 0;

        Stack<Integer> stack = new Stack<>();

        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == '(') {
                stack.push(i);
                continue;
            }

            // never opened
            if (stack.isEmpty()) continue;

            // sibiling continuation
            if (i + 1 < s.length() && s.charAt(i + 1) == '(') {
                best = Math.max(best, i - stack.peek() + 1);
                i++;
            } else {
                best = Math.max(best, i - stack.pop() + 1);
            }
        }

        return best;
    }
}
