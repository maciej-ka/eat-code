// https://leetcode.com/problems/remove-invalid-parentheses/submissions/2165614989/?envType=daily-question&envId=2026-10-07
import java.util.*;

class Solution {

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

        var left = s.subSequence(0, ileft).toString();
        var middle = s.subSequence(ileft, iright).toString();
        var right = s.subSequence(iright, s.length()).toString();

        var result = new ArrayList<String>();
        var validLeft = solveLeft(left);
        var validRight = solveRight(right);

        for (var l: validLeft)
            for (var r: validRight)
                result.add(l + middle + r);

        return result;
    }

    private Set<String> leftSeen = new HashSet<>();

    private List<String> solveLeft(String s) {
        leftSeen.add(s);

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
            return List.of(s);
        }

        var result = new ArrayList<String>();
        var last = 'a';
        for (int k = 0; k <= i; k++) {
            var c = s.charAt(k);
            if (c == ')' && last != ')') {
                var removed = s.substring(0, k) + s.substring(k + 1);
                if (!leftSeen.contains(removed))
                    result.addAll(solveLeft(removed));
            }
            last = c;
        }

        return result;
    }

    private Set<String> rightSeen = new HashSet<>();

    private List<String> solveRight(String s) {
        rightSeen.add(s);

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
            return List.of(s);
        }

        var result = new ArrayList<String>();
        var last = 'a';
        for (int k = s.length() - 1; k >= i; k--) {
            var c = s.charAt(k);
            if (c == '(' && last != '(') {
                var removed = s.substring(0, k) + s.substring(k + 1);
                if (!rightSeen.contains(removed))
                    result.addAll(solveRight(removed));
            }
            last = c;
        }

        return result;
    }
}
