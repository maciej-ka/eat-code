// https://leetcode.com/problems/minimum-sum-of-squared-difference/submissions/2168068048/?envType=daily-question&envId=2026-10-10
import java.util.*;

class Solution {
    public long minSumSquareDiff(int[] nums1, int[] nums2, int k1, int k2) {
        var map = new TreeMap<Long, Long>((a, b) -> Long.compare(b, a));
        for (int i = 0; i < nums1.length; i++) {
            long diff = Math.abs(nums1[i] - nums2[i]);
            long count = map.getOrDefault(diff, 0L);
            map.put(diff, count + 1);
        }

        long have = k1 + k2;

        var iterator = map.entrySet().iterator();
        var first = iterator.next();
        long value = first.getKey();
        long count = first.getValue();
        long result = 0;

        while (value > 0) {
            var entry = iterator.hasNext() ? iterator.next() : Map.entry(0L, 0L);
            long goal = entry.getKey();
            long need = (value - goal) * count;

            if (need <= have) {
                have -= need;
                value = goal;
                count += entry.getValue();
                continue;
            }

            // not enough
            value -= have / count;
            long rest = have % count;
            result += value * value * (count - rest);
            result += (value - 1) * (value - 1) * rest;
            result += goal * goal * entry.getValue();
            break;
        }

        while (iterator.hasNext()) {
            var entry = iterator.next();
            result += entry.getKey() * entry.getKey() * entry.getValue();
        }

        return result;
    }
}
