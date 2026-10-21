// https://leetcode.com/problems/minimum-operations-to-sort-a-string/submissions/2177822632/?envType=daily-question&envId=2026-10-21
import java.util.*;

class Solution {
    public int minOperations(String s) {
        char max = 'a';
        char min = 'z';
        char last = s.charAt(0);
        int maxCount = 0;
        int minCount = 0;
        boolean sorted = true;

        for (char c: s.toCharArray()) {
            if (c > max) { max = c; maxCount = 0; }
            if (c < min) { min = c; minCount = 0; }
            if (c == max) maxCount++;
            if (c == min) minCount++;
            if (c < last) sorted = false;
            last = c;
        }

        if (sorted) return 0;
        if (s.length() == 2) return -1;
        if (s.charAt(0) == max && s.charAt(s.length() - 1) == min && maxCount == 1 && minCount == 1) return 3;
        if (s.charAt(0) == min || s.charAt(s.length() - 1) == max) return 1;
        return 2;
    }
}
