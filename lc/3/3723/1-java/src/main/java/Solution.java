// https://leetcode.com/problems/maximize-sum-of-squares-of-digits/submissions/2171913370/?envType=daily-question&envId=2026-10-14
import java.util.*;

class Solution {
    public String maxSumOfSquares(int num, int sum) {
        if (sum > num * 9) return "";
        char[] digits = new char[]{'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'};
        char[] chars = new char[num];
        for (int i = 0; i < num; i++) {
            int digit = Math.min(9, sum);
            sum -= digit;
            chars[i] = digits[digit];
        }
        return new String(chars);
    }
}
