// https://leetcode.com/problems/valid-parenthesis-string/submissions/2162508564/?envType=daily-question&envId=2026-10-04
import java.util.*;

class Solution {
    public boolean checkValidString(String s) {
        int min = 0;
        int max = 0;

        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(') {
                min++;
                max++;
            } else if (c == ')') {
                if (max == 0) return false;
                min = Math.max(0, min - 1);
                max--;
            } else { // *
                min = Math.max(0, min - 1);
                max++;
            }
        }

        return min == 0;
    }
}
