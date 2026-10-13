import java.util.ArrayList;
import java.util.List;

// https://leetcode.com/problems/pascals-triangle/submissions/1719525245/?envType=daily-question&envId=2025-08-01
// 82.66%
// 7.56%
class Solution {
    public List<List<Integer>> generate(int numRows) {
        List<List<Integer>> result = new ArrayList<>(numRows);
        List<Integer> last = List.of(1);
        result.add(0, last);
        int i = 1;

        while (i < numRows) {
            List<Integer> row = new ArrayList<>(i + 1);
            row.add(0, 1);
            for (int k = 1; k < i; k++) {
                row.add(k, last.get(k) + last.get(k - 1));
            }
            row.add(i, 1);

            result.add(i, row);
            last = row;
            i += 1;
        }

        return result;
    }
}
