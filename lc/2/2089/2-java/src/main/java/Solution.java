// https://leetcode.com/problems/find-target-indices-after-sorting-array/submissions/2176101893/?envType=daily-question&envId=2026-10-19
import java.util.*;

class Solution {
    public List<Integer> targetIndices(int[] nums, int target) {
        int smaller = 0;
        int count = 0;
        for (int num: nums)
            if (num < target)
                smaller++;
            else if (num == target)
                count++;

        List<Integer> res = new ArrayList<Integer>(count);
        for (int i = 0; i < count; i++)
            res.add(smaller + i);

        return res;
    }
}
