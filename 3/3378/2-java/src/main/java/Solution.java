// https://leetcode.com/problems/count-connected-components-in-lcm-graph/submissions/2170000732/
import java.util.*;

class Solution {
    int[] groups;

    public int countComponents(int[] nums, int threshold) {
        var res = 0;
        var max = Math.min(200001, threshold);
        groups = new int[max + 1];

        for (var base: nums) {
            if (base > max) {
                res++;
                continue;
            }

            if (groups[base] != 0)
                continue;

            groups[base] = base;
            res++;

            var num = base << 1;
            var group = base;
            while (num <= max) {
                if (groups[num] == 0) {
                    groups[num] = group;
                } else {
                    var another = find(groups[num]);
                    if (group != another) {
                        groups[group] = another;
                        group = another;
                        res--;
                    }
                }
                num += base;
            }
        }

        return res;
    }

    int find(int num) {
        if (groups[num] != num)
            groups[num] = find(groups[num]);
        return groups[num];
    }
}
