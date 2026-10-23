// https://leetcode.com/problems/minimum-operations-to-sort-a-permutation/submissions/2179804402/?envType=daily-question&envId=2026-10-23
import java.util.*;

class Solution {
    public int minOperations(int[] nums) {
        boolean isAsc = true;
        int izero = 0;
        int len = nums.length;

        // check is sorted ascending
        int last = nums[len - 1];
        for (int i = 0; i < len; i++) {
            if (nums[i] == 0)
                izero = i;
            else if (nums[i] < last) {
                isAsc = false;
                break;
            }
            last = nums[i];
        }

        // check is sorted descending
        if (!isAsc) {
            last = nums[0];
            for (int i = len - 1; i >= 0; i--) {
                if (nums[i] == 0)
                    izero = i;
                else if (nums[i] < last)
                    return -1;
                last = nums[i];
            }
        }

        return isAsc
            ? Math.min(izero, len - izero + 2)
            : Math.min(izero + 2, len - izero);
    }
}
