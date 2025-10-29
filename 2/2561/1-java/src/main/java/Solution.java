// https://leetcode.com/problems/rearranging-fruits/submissions/1721844781/?envType=daily-question&envId=2025-08-02

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;

class Solution {
    public long minCost(int[] basket1, int[] basket2) {

        Arrays.sort(basket1);
        Arrays.sort(basket2);

        // merge baskets
        int[] all = new int[basket1.length + basket2.length];
        int i1 = 0;
        int i2 = 0;
        while (i1 + i2 < all.length) {
            // one empty basket
            if (i2 >= basket2.length) {
                all[i1 + i2] = basket1[i1++];
                continue;
            }
            if (i1 >= basket1.length) {
                all[i1 + i2] = basket2[i2++];
                continue;
            }
            // two nonempty baskets
            if (basket1[i1] < basket2[i2]) {
                all[i1 + i2] = basket1[i1++];
            } else {
                all[i1 + i2] = basket2[i2++];
            }
        }
        int min = all[0];

        // collect counts
        var counts = new HashMap<Integer, Integer>();
        for (int num: basket1) {
            counts.put(num, counts.getOrDefault(num, 0) + 1);
        }
        for (int i = 0; i < all.length; i += 2) {
            counts.put(all[i], counts.getOrDefault(all[i], 0) - 1);
            // detect impossible solution
            if (all[i] != all[i + 1]) {
                return -1;
            }
        }

        // convert counts to need / have
        var need = new ArrayList<Integer>();
        var have = new ArrayList<Integer>();
        for (int num: counts.keySet()) {
            if (counts.get(num) > 0) {
                have.add(num);
            }
            if (counts.get(num) < 0) {
                need.add(num);
                // convert count to absolute
                counts.put(num, -1 * counts.get(num));
            }
        }
        have.sort(Comparator.naturalOrder());
        need.sort(Comparator.reverseOrder());

        // get result from need / have
        long result = 0;
        while(need.size() > 0) {
            // get next need
            var xn = need.get(0);
            var count = counts.get(xn);
            if (count == 1) {
                need.remove(0);
            } else {
                counts.put(xn, count - 1);
            }
            // get next have
            var xh = have.get(0);
            count = counts.get(xh);
            if (count == 1) {
                have.remove(0);
            } else {
                counts.put(xh, count - 1);
            }

            // cost of straight exchange
            var cost1 = Math.min(xn, xh);
            // cost of swap using cheapest
            var cost2 = min << 1;

            result += Math.min(cost1, cost2);
        }

        return result;
    }
}
