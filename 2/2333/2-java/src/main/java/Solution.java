// https://leetcode.com/problems/minimum-sum-of-squared-difference/submissions/2168145217/?envType=daily-question&envId=2026-10-10
import java.util.*;

class Solution {
    public long minSumSquareDiff(int[] nums1, int[] nums2, int k1, int k2) {
        var counts = new int[100001];
        for (int i = 0; i < nums1.length; i++) {
            var diff = Math.abs(nums1[i] - nums2[i]);
            counts[diff]++;
        }

        var have = k1 + k2;
        for (int i = 100000; i > 0; i--) {
            var take = Math.min(have, counts[i]);
            counts[i] -= take;
            counts[i - 1] += take;
            have -= take;
        }

        long result = 0;
        for (int i = 0; i <= 100000; i++) {
            result += (long) counts[i] * (long) i * i;
        }

        return result;
    }
}
