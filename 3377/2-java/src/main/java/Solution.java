// https://leetcode.com/problems/fruits-into-baskets-ii/submissions/1724307335/?envType=daily-question&envId=2025-08-05
// 100.00%
// 62.30%
class Solution {
    public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
        int unplaced = 0;
        for (int fruit: fruits) {
            int b = 0;
            for (; b < baskets.length && baskets[b] < fruit; b++);
            if (b >= baskets.length) {
                unplaced++;
            } else {
                baskets[b] = -1;
            }
        }
        return unplaced;
    }
}
