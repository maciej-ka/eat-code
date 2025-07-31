import java.util.Arrays;

class Solution {
    public int partitionArray(int[] nums, int k) {
        if (nums.length == 0) {
            return 0;
        }

        Arrays.sort(nums);
        // System.out.println(Arrays.toString(nums));

        int result = 1;
        int active = nums[0];

        for (int num : nums) {
            if (num - active > k) {
                result++;
                active = num;
            }
        }

        return result;
    }
}
