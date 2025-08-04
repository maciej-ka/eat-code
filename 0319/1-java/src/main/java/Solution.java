import java.util.Arrays;

// https://leetcode.com/problems/bulb-switcher/submissions/1723382284/
// 100%
// 80.96%
class Solution {
    public int bulbSwitch(int n) {
        if (n < 1) return ;
        int result = 1;
        int step = 3;
        while (step < n) {
            result++;
            n -= step;
            step += 2;
        }
        return result;
    }
}

