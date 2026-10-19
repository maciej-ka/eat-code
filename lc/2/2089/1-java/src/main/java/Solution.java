// https://leetcode.com/problems/find-target-indices-after-sorting-array/submissions/2176089961/?envType=daily-question&envId=2026-10-19
import java.util.*;

class Solution {
    public List<Integer> targetIndices(int[] nums, int target) {
        Arrays.sort(nums);
        var res = new ArrayList<Integer>();
        for (int i = 0; i < nums.length; i++)
            if (nums[i] == target)
                res.add(i);
        return res;
    }
}
