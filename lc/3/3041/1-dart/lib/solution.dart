// https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification/submissions/2200039870/?envType=daily-question&envId=2026-11-20
import 'dart:math';

class Solution {
  int maxSelectedElements(List<int> nums) {
    var counts = <int, int>{};
    var mx = 0;
    for (var num in nums) {
      mx = max(mx, num);
      counts[num] = (counts[num] ?? 0) + 1;
    }

    var best = 1;

    var current = 0;
    // elements that can be shift added
    var left = 0;
    var nextLeft = 0;
    // is double present
    var double = 0;

    for (var num = 1; num <= mx + 1; num++) {
      // no gap
      if (counts[num] != null) {
        current++;
        nextLeft++;
        if (counts[num]! > 1) double = 1;
        continue;
      }

      // gap of size 2+
      if (counts[num + 1] == null) {
        best = max(best, current + left + double);
        current = nextLeft = left = double = 0;
        continue;
      }

      // gap of size 1, double present
      if (double > 0) {
        double = 0;
        nextLeft = 0;
        current++;
        continue;
      }

      // gap of size 1, no double present
      // a) glue existing with left
      best = max(best, current + left);
      // b) glue existing with right
      left = nextLeft;
      current = 0;
      nextLeft = 0;
    }

    return best;
  }
}
