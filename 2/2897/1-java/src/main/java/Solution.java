// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares/submissions/2170547896/?envType=daily-question&envId=2026-10-12
import java.util.*;

class Solution {
    static final int BITS = 30;
    static final int MOD = 1000000007;

    public int maxSum(List<Integer> nums, int k) {
        var counts = new int[BITS];
        for (var num: nums)
            for (int i = 0; i < BITS; i++)
                if ((num & 1 << i) != 0)
                    counts[i]++;

        var res = 0;
        for (int j = 0; j < k; j++) {
            var n = 0;
            for (int i = 0; i < BITS; i++) {
                if (counts[i] > 0) {
                    n += 1 << i;
                    counts[i]--;
                }
            }
            res = (res + mul(n, n)) % MOD;
        }

        return res;
    }

    int mul(int a, int b) {
        int res = 0;
        while (b > 0) {
            if ((b & 1) == 1)
                res = (res + a) % MOD;
            b >>= 1;
            a = (a << 1) % MOD;
        }
        return res;
    }
}
