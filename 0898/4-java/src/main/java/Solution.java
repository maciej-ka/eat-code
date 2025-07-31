import java.util.TreeSet;

// https://leetcode.com/problems/bitwise-ors-of-subarrays/submissions/1718783268/?envType=daily-question&envId=2025-07-31
// 12.30%
// 5.74%
class Solution {
    public int subarrayBitwiseORs(int[] arr) {
        int len = arr.length;

        var sums = new TreeSet<Integer>();
        for (int num: arr) {
            sums.add(num);
        }

        while (len > 1) {
            int k = 0;
            for (int i = 1; i < len; i++) {
                arr[k] = arr[i] | arr[i - 1];
                if (k == 0 || arr[k] != arr[k - 1]) {
                    k += 1;
                }
            }
            len = k;
            for (int i = 0; i < len; i++) {
                sums.add(arr[i]);
            }
        }

        return sums.size();
    }
}
