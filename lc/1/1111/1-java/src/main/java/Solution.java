// https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/submissions/2158415741/?envType=daily-question&envId=2026-09-30

class Solution {
    public int[] maxDepthAfterSplit(String seq) {
        int depth = 1;
        int[] result = new int[seq.length()];

        for (int i = 0; i < seq.length(); i++) {
            if (seq.charAt(i) == '(') {
                depth++;
                result[i] = depth % 2;
            } else {
                result[i] = depth % 2;
                depth--;
            }
        }

        return result;
    }
}
