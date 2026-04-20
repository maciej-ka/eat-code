// https://leetcode.com/problems/two-furthest-houses-with-different-colors/submissions/1983950176/?envType=daily-question&envId=2026-04-20
namespace Solution;

public class Solution {
    public int MaxDistance(int[] colors) {
        int last = colors.Length - 1;
        for (int i = 0; i < last; i++) {
            if (colors[0] != colors[last - i] || colors[i] != colors[last]) {
                return last - i;
            }
        }
        return -1;
    }
}
