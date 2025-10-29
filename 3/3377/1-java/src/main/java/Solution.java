// https://leetcode.com/problems/fruits-into-baskets-ii/submissions/1724296607/?envType=daily-question&envId=2025-08-05

class Solution {
    private int[] nextFree;

    private int getNextFree(int i) {
        if (i >= nextFree.length) return -1;
        if (nextFree[i] == -1) return -1;
        if (nextFree[i] == 0) return i;
        return nextFree[i] = getNextFree(nextFree[i]);
    };

    private void removeFree(int i) {
        nextFree[i] = getNextFree(i + 1);
    }

    public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
        nextFree = new int[fruits.length];
        int unplaced = 0;
        for (int fruit: fruits) {
            int b = getNextFree(0);
            while (b != -1 && baskets[b] < fruit) {
                b = getNextFree(b + 1);
            }

            if (b == -1) {
                unplaced++;
            } else {
                removeFree(b);
            }
        }
        return unplaced;
    }
}
