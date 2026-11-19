// https://leetcode.com/problems/minimum-score-by-changing-two-elements/submissions/2199611913/?envType=daily-question&envId=2026-11-19

import 'dart:math';

class Solution {
  int minimizeSum(List<int> nums) {
    var len = nums.length;
    if (len <= 3) return 0;

    nums.sort();
    var res = nums[len - 1] - nums[0];
    for (var i = 0; i < len; i++) {
      for (var k = len - 3 + i; k < len; k++) {
        res = min(res, nums[k] - nums[i]);
      }
    }

    return res;
  }
}
