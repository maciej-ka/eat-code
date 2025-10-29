// https://leetcode.com/problems/fruits-into-baskets-ii/submissions/1724307335/

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
