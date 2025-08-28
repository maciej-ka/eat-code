// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/?envType=daily-question&envId=2025-08-26
// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/submissions/1748875375/?envType=daily-question&envId=2025-08-26

class Solution {
  public int areaOfMaxDiagonal(int[][] dimensions) {
    // diagonal to power 2
    int maxScore = 0;
    int maxArea = 0;

    for (int[] pair: dimensions) {
      int score = pair[0] * pair[0] + pair[1] * pair[1];
      if (score < maxScore) { continue; }

      int area = pair[0] * pair[1];
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
