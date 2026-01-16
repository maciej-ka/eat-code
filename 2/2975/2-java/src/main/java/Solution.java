// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/submissions/1887181782/?envType=daily-question&envId=2026-01-16

import java.util.Arrays;

class Solution {
  public int maximizeSquareArea(int m, int n, int[] hFences, int[] vFences) {

    var xlen = vFences.length + 2;
    var xs = Arrays.copyOf(vFences, xlen);
    xs[xlen - 1] = 1;
    xs[xlen - 2] = n;
    Arrays.sort(xs);

    var ylen = hFences.length + 2;
    var ys = Arrays.copyOf(hFences, ylen);
    ys[ylen - 1] = 1;
    ys[ylen - 2] = m;
    Arrays.sort(ys);

    long best = -1;

    for (var x: xs) {
      for (var y: ys) {
        var ix = xs.length - 1;
        var iy = ys.length - 1;
        while (true) {
          long w = xs[ix] - x;
          long h = ys[iy] - y;
          long area = w * h;
          if (area == 0 || area < best) { break; }
          if (w == h) {
            best = area;
            break;
          }
          if (w > h) { ix--; } else { iy--; }
        }
      }
    }

    return (int)(best % 1000000007);
  }
}
