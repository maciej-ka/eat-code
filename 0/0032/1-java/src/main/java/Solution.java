// https://leetcode.com/problems/longest-valid-parentheses/submissions/2161129460/?envType=daily-question&envId=2026-10-03
import java.util.*;

class Solution {
    public int longestValidParentheses(String s) {
        int best = 0;

        Map<Integer, Integer> levelStart = new HashMap<>(1000);
        int level = 0;

        for (int i = 0; i < s.length(); i++) {
            // open
            if (s.charAt(i) == '(') {
                level++;
                levelStart.put(level, i);
                continue;
            }

            // close
            Integer started = levelStart.get(level);
            level--;

            // never started
            if (started == null) continue;

            // range done
            int length = i - started + 1;
            best = Math.max(length, best);

            // sibling continuation
            if (i + 1 < s.length() && s.charAt(i + 1) == '(') {
                i++;
                level++;
            }

        }

        return best;
    }
}
