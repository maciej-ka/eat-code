// wrong
import java.util.*;

class Solution {
    public int sumCounts(int[] nums) {
        long delta = 0;
        long res = 0;
        long base = 0;
        final long mod = 1000000007;
        final int len = nums.length;
        int[] last = new int[100001];

        for (int i = 0; i < len; i++) {
            final int val = nums[i];
            final int seen = last[val];

            if (seen > 0) {
                final int dist = i - seen + 1;
                delta += dist * dist;
            } else {
                delta += base * 2 + i + 1;
            }
            delta %= mod;
            last[val] = i + 1;

            res += delta;
            res %= mod;

            base += i + 1 - seen;
        }

        return (int) res;
    }
}
