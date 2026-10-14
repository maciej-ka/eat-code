// https://leetcode.com/problems/maximize-sum-of-squares-of-digits/submissions/2171939472/?envType=daily-question&envId=2026-10-14
import java.util.*;

class Solution {
    public String maxSumOfSquares(int num, int sum) {
        if (sum > num * 9) return "";
        StringBuilder builder = new StringBuilder(num);

        int nines = sum / 9;
        builder.repeat('9', nines);
        if (nines < num) builder.append(sum % 9);
        int zeroes = num - nines - 1;
        if (zeroes > 0) builder.repeat('0', zeroes);

        return builder.toString();
    }
}
