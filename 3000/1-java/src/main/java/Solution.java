// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/?envType=daily-question&envId=2025-08-26
// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/submissions/1748669207/?envType=daily-question&envId=2025-08-26

class Solution {
    public int areaOfMaxDiagonal(int[][] dimensions) {
        // diagonal to power 2
        int maxScore = 0;
        int maxArea = 0;

        for (int i = 0; i < dimensions.length; i++) {
            int score = dimensions[i][0] * dimensions[i][0] + dimensions[i][1] * dimensions[i][1];
            if (score < maxScore) { continue; }

            int area = dimensions[i][0] * dimensions[i][1];
            if (score > maxScore) { 
                maxScore = score;
                maxArea = area;
            }

            // score is equal to maxScore
            if (area > maxArea) { maxArea = area; }
        }

        return maxArea;
    }
}
