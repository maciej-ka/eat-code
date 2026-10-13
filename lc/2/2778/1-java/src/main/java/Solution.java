// https://leetcode.com/problems/sum-of-squares-of-special-elements/submissions/2169305295/?envType=daily-question&envId=2026-10-11
import java.util.*;

class Solution {
    public int sumOfSquares(int[] nums) {
        var res = 0;
        var len = nums.length;
        for (int i = 0; i < len; i++) {
            if (len % (i + 1) == 0) {
                res += nums[i] * nums[i];
            }
        }

        return res;
    }
}
