// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/submissions/1887074670/?envType=daily-question&envId=2026-01-16

import java.util.ArrayList;
import java.util.Collections;

class Solution {
  public int maximizeSquareArea(int m, int n, int[] hFences, int[] vFences) {
    var xs = new ArrayList<Integer>(vFences.length + 2);
    xs.add(1);
    xs.add(n);
    for (var x: vFences) { xs.add(x); }
    Collections.sort(xs);

    var ys = new ArrayList<Integer>(hFences.length + 2);
    ys.add(1);
    ys.add(m);
    for (var y: hFences) { ys.add(y); }
    Collections.sort(ys);

    long best = -1;

    for (var x: xs) {
      for (var y: ys) {
        var ix = xs.size() - 1;
        var iy = ys.size() - 1;
        while (true) {
          long w = xs.get(ix) - x;
          long h = ys.get(iy) - y;
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
