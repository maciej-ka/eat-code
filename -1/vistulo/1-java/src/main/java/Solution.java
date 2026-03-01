import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

class Solution {

  public static List<Integer> processArray(int[] input) {
    var skip = new ArrayList<Integer>();
    for (var num: input) {
      if (num > 0) {
        skip.add(num);
      }
    }

    Collections.sort(skip);

    var result = new ArrayList<Integer>();
    var i = 0;
    var k = 0;
    for (var num: input) {
      if (num < 0) {
        i++;
        if (k < skip.size() && i == skip.get(k)) {
          while (k < skip.size() && i == skip.get(k)) {
            k++;
          }
        } else {
          result.add(num);
        }
      }
    }

    return result;
  }

  public int solve(int[] nums) {
    return nums.length;
  }
}
