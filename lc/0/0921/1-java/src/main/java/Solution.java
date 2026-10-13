// thttps://leetcode.com/problems/minimum-add-to-make-parentheses-valid/submissions/2164004422/?envType=daily-question&envId=2026-10-06
import java.util.*;

class Solution {
    public int minAddToMakeValid(String s) {
        var adds = 0;
        var opened = 0;

        for (char c: s.toCharArray()) {
            if (c == '(') {
                opened++;
            } else if (opened != 0) {
                opened--;
            } else {
                adds++;
            }
        }

        return opened + adds;
    }
}
