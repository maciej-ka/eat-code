// https://leetcode.com/problems/bulb-switcher/submissions/1723382284/

class Solution {
    public int bulbSwitch(int n) {
        if (n < 1) return 0;
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

