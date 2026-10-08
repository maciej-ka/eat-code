// https://leetcode.com/problems/remove-outermost-parentheses/submissions/2166080063/?envType=daily-question&envId=2026-10-08
import java.util.*;

class Solution {
    public String removeOuterParentheses(String s) {
        int level = 0;
        var builder = new StringBuilder();
        for (var c: s.toCharArray()) {
            if (c == '(') {
                if (level > 0) builder.append(c);
                level++;
            } else {
                level--;
                if (level > 0) builder.append(c);
            }
        }
        return builder.toString();
    }
}
