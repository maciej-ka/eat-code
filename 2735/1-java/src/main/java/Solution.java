// https://leetcode.com/problems/collecting-chocolates/submissions/1724455579/
// 93.65%
// 100.00%
class Solution {
    public long minCost(int[] nums, int x) {

        if (nums.length == 1) return nums[0];

        long sum = 0;
        for (int num: nums) sum += num;
        long best = sum;

        for (int i = 0; i < nums.length; i++) {
            sum += x;
            int old1 = nums[1];
            // skip 0
            for (int k = 1; k < nums.length; k++) {
                int alt = nums[(k + 1) % nums.length];
                if (nums[k] > alt) {
                    sum -= nums[k] - alt;
                    nums[k] = alt;
                }
            }
            // handle skipped 0
            if (nums[0] > old1) {
                sum -= nums[0] - old1;
                nums[0] = old1;
            }

            best = Math.min(best, sum);
        }

        return best;
    }
}
