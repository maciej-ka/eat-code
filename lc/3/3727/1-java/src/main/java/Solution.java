// https://leetcode.com/problems/maximum-alternating-sum-of-squares/submissions/2171050292/?envType=daily-question&envId=2026-10-13
import java.util.*;

class Solution {
    public long maxAlternatingSum(int[] nums) {
        int len = nums.length;
        for (int i = 0; i < len; i++)
            nums[i] *= nums[i];
        Arrays.sort(nums);

        long res = 0;
        int half = len / 2;
        for (int i = 0; i < half; i++)
            res += nums[i + half] - nums[i];
        if (len % 2 == 1)
            res += nums[len - 1];

        return res;
    }

}
