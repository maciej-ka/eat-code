// https://leetcode.com/problems/calculate-digit-sum-of-a-string/submissions/2197433711/?envType=daily-question&envId=2026-11-16

import 'dart:math';

class Solution {
  String digitSum(String s, int k) {
    while (s.length > k) {
      var round = "";

      for (int i = 0; i < s.length; i += k) {
        var group = s.substring(i, min(s.length, i + k));
        var sum = 0;
        for (int j = 0; j < group.length; j++) {
          sum += int.parse(group[j]);
        }
        round += "$sum";
      }

      s = round;
    }

    return s;
  }
}
