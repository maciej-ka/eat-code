// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/submissions/2167076198/?envType=daily-question&envId=2026-10-09
import java.util.*;

class Solution {
    public int minInsertions(String s) {
        int res = 0;
        int opened = 0;
        char[] chars = s.toCharArray();
        int len = s.length();

        for (int i = 0; i < len; i++) {
            if (chars[i] == '(')
                opened++;
            else { // ')'
                if (opened == 0)
                    res++;
                else
                    opened--;

                if (i + 1 == len)
                    res++;
                else {
                    if (chars[i + 1] == ')')
                        i++;
                    else
                        res++;
                }
            }
        }

        return res + opened * 2;
    }
}
