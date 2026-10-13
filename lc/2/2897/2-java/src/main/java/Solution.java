// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares/submissions/2170580166/?envType=daily-question&envId=2026-10-12
import java.util.*;

class Solution {
    static final int BITS = 30;
    static final int MOD = 1000000007;

    public int maxSum(List<Integer> nums, int k) {
        int[] counts = new int[BITS];
        int len = nums.size();
        for (int j = 0; j < len; j++) {
            int num = nums.get(j);
            for (int i = 0; i < BITS; i++)
                counts[i] += num >> i & 1;
        }

        long res = 0;
        for (int j = 0; j < k; j++) {
            long n = 0;
            for (int i = 0; i < BITS; i++)
                if (counts[i] > 0) {
                    n |= 1 << i;
                    counts[i]--;
                }
            res = (res + n * n) % MOD;
        }

        return (int) res;
    }
}
