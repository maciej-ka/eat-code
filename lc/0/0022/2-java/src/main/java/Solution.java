// https://leetcode.com/problems/generate-parentheses/submissions/2160537581/?envType=daily-question&envId=2026-10-02

import java.util.*;

class Solution {
    private List<String> result;

    public List<String> generateParenthesis(int n) {
        result = new ArrayList<>();
        build("", 0, 0, n);
        return result;
    }

    public void build(String s, int open, int closed, int n) {
        if (open == n && closed == n) {
            result.add(s);
            return;
        }

        if (open < n)
            build(s + "(", open + 1, closed, n);

        if (closed < open)
            build(s + ")", open, closed + 1, n);
    }
}
