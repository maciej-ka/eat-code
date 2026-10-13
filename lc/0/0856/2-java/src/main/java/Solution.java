// thttps://leetcode.com/problems/score-of-parentheses/submissions/2162954446/?envType=daily-question&envId=2026-10-05
import java.util.*;

class Solution {
    public int scoreOfParentheses(String s) {
        var score = 0;
        var depth = 0;

        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == '(') {
                depth++;
            } else {
                depth--;
                if (s.charAt(i - 1) == '(')
                    score += 1 << depth;
            }
        }

        return score;
    }
}
