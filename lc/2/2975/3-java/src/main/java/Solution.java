// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/submissions/1887214969/?envType=daily-question&envId=2026-01-16

import java.util.HashSet;

class Solution {
  private HashSet<Integer> widths;
  private int best = -1;

  private void check(int height) {
    if (widths.contains(height) && height > best) {
      best = height;
    }
  }

  public int maximizeSquareArea(int m, int n, int[] hFences, int[] vFences) {
    var xlen = vFences.length;
    widths = new HashSet<Integer>();
    widths.add(n - 1);
    for (var i = 0; i < xlen; i++) {
      widths.add(vFences[i] - 1);
      widths.add(n - vFences[i]);
      for (var j = i + 1; j < xlen; j++) {
        widths.add(Math.abs(vFences[i] - vFences[j]));
      }
    }

    var ylen = hFences.length;
    check(m - 1);
    for (var i = 0; i < ylen; i++) {
      check(hFences[i] - 1);
      check(m - hFences[i]);
      for (var j = i + 1; j < ylen; j++) {
        check(Math.abs(hFences[i] - hFences[j]));
      }
    }

    if (best > 0) {
      return (int)((long)best * (long)best % 1000000007);
    } else {
      return -1;
    }
  }
}

