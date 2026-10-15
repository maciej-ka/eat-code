// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-i/submissions/2172854214/?envType=daily-question&envId=2026-10-15
import java.util.*;

class Solution {
    public int sumCounts(List<Integer> nums) {
        int res = 0;
        int len = nums.size();
        int[] counts = new int[101];
        int distinct = 0;

        for (int i = 0; i < len; i++) {
            if (i % 2 == 0) {
                for (int k = i; k < len; k++) {
                    if (++counts[nums.get(k)] == 1) distinct++;
                    res += distinct * distinct;
                }
                if (--counts[nums.get(i)] == 0) distinct--;
            } else {
                for (int k = len - 1; k >= i; k--) {
                    res += distinct * distinct;
                    if (--counts[nums.get(k)] == 0) distinct--;
                }
            }
        }

        return res;
    }
}
