// https://leetcode.com/problems/bitwise-ors-of-subarrays/submissions/1718468075/?envType=daily-question&envId=2025-07-31

import java.util.HashSet;

class Solution {
    public int subarrayBitwiseORs(int[] arr) {
        var sums = new HashSet<Integer>();
        for (int num: arr) {
            sums.add(num);
        }

        int[] next;

        while (arr.length > 1) {
            // count sibiling duplicates
            int duplicates = 0;
            for (int i = 1; i < arr.length; i++) {
                if (arr[i] == arr[i-1]) {
                    duplicates += 1;
                }
            }

            next = new int[arr.length - 1 - duplicates];
            int k = 0;

            for (int i = 1; i < arr.length; i++) {
                if (arr[i] != arr[i - 1]) {
                    next[k] = arr[i] | arr[i - 1];
                    k += 1;
                }
            }
            arr = next;

            for (int num: arr) {
                sums.add(num);
            }
        }

        return sums.size();
    }
}
