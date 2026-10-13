// https://leetcode.com/problems/remove-invalid-parentheses/submissions/2165778149/?envType=daily-question&envId=2026-10-07
import java.util.*;

class Solution {

    private List<String> leftSolutions = new ArrayList<>();
    private List<String> rightSolutions = new ArrayList<>();

    public List<String> removeInvalidParentheses(String s) {
        var ileft = 0;
        var opened = 0;

        for (int i = 0; i < s.length(); i++) {
            var c = s.charAt(i);
            if (c == '(') {
                opened++;
            } else if (c == ')') {
                if (opened == 0) {
                    ileft = i + 1;
                } else {
                    opened--;
                }
            }
        }

        var iright = s.length();
        opened = 0;

        for (int i = s.length() - 1; i >= 0; i--) {
            var c = s.charAt(i);
            if (c == ')') {
                opened++;
            } else if (c == '(') {
                if (opened == 0) {
                    iright = i;
                } else {
                    opened--;
                }
            }
        }

        var left = s.substring(0, ileft);
        var middle = s.substring(ileft, iright);
        var right = s.substring(iright, s.length());

        solveLeft(left, 0);
        solveRight(right, right.length() - 1);
        var result = new ArrayList<String>(leftSolutions.size() * rightSolutions.size());

        for (var l: leftSolutions)
            for (var r: rightSolutions)
                result.add(l + middle + r);

        return result;
    }

    private void solveLeft(String s, int cutFrom) {
        int i = 0;
        int opened = 0;

        while (i < s.length()) {
            var c = s.charAt(i);
            if (c == '(') {
                opened++;
            } else if (c == ')') {
                if (opened == 0) {
                    break;
                } else {
                    opened--;
                }
            }
            i++;
        }

        if (i == s.length()) {
            leftSolutions.add(s);
            return;
        }

        var last = 'a';
        for (int k = cutFrom; k <= i; k++) {
            var c = s.charAt(k);
            if (c == ')' && last != ')') {
                var removed = s.substring(0, k) + s.substring(k + 1);
                solveLeft(removed, k);
            }
            last = c;
        }
    }

    private void solveRight(String s, int cutFrom) {
        int i = s.length() - 1;
        int opened = 0;

        while (i >= 0) {
            var c = s.charAt(i);
            if (c == ')') {
                opened++;
            } else if (c == '(') {
                if (opened == 0) {
                    break;
                } else {
                    opened--;
                }
            }
            i--;
        }

        if (i == -1) {
            rightSolutions.add(s);
            return;
        }

        var last = 'a';
        for (int k = cutFrom; k >= i; k--) {
            var c = s.charAt(k);
            if (c == '(' && last != '(') {
                var removed = s.substring(0, k) + s.substring(k + 1);
                solveRight(removed, k - 1);
            }
            last = c;
        }
    }
}
