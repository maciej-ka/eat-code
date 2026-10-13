// https://leetcode.com/problems/generate-parentheses/submissions/2160273867/?envType=daily-question&envId=2026-10-02

import java.util.*;

class Solution {
    private HashMap<Integer, List<String>> memo = new HashMap<>();
    public List<String> generateParenthesis(int n) {
        if (n == 0) return List.of("");
        if (n == 1) return List.of("()");
        if (memo.containsKey(n)) return memo.get(n);

        List<String> list = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            List<String> in = generateParenthesis(n - 1 - i);
            List<String> out = generateParenthesis(i);
            for (String sin: in) {
                for (String sout: out) {
                    list.add("(" + sin + ")" + sout);
                }
            }
        }
        memo.put(n, list);
        return list;
    }
}
