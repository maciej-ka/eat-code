// too slow
import java.util.*;

class Solution {
    public int sumCounts(int[] nums) {
        long res = 0;
        long distinct = 0;
        final long MOD = 1000000007;

        int len = nums.length;
        int[] counts = new int[100001];

        for (int i = 0; i < len; i++) {
            if (i % 2 == 0) {
                for (int k = i; k < len; k++) {
                    if (++counts[nums[k]] == 1) distinct++;
                    res += distinct * distinct;
                    res %= MOD;
                }
                if (--counts[nums[i]] == 0) distinct--;
            } else {
                for (int k = len - 1; k >= i; k--) {
                    res += distinct * distinct;
                    res %= MOD;
                    if (--counts[nums[k]] == 0) distinct--;
                }
            }
        }

        return (int) res;
    }
}
