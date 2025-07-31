import java.util.HashSet;

// seems to work, but it's too slow
class Solution {
    public int subarrayBitwiseORs(int[] arr) {
        var sums = new HashSet<Integer>();
        for (int num: arr) {
            sums.add(num);
        }

        int[] next;

        while (arr.length > 1) {
            next = new int[arr.length - 1];
            for (int i = 0; i < next.length; i++) {
                next[i] = arr[i] | arr[i + 1];
            }
            arr = next;

            for (int num: arr) {
                sums.add(num);
            }
        }

        return sums.size();
    }
}
