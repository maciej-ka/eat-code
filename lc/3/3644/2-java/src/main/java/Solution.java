// https://leetcode.com/problems/maximum-k-to-sort-a-permutation/submissions/2180041963/?envType=daily-question&envId=2026-10-23
import java.util.*;

class Solution {
    public int sortPermutation(int[] nums) {
        int res = -1;

        for (int i = 0; i < nums.length; i++)
            if (i != nums[i])
                res &= nums[i];

        return Math.max(0, res);
    }
}
