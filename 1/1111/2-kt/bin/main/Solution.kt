// https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/submissions/2158449643/?envType=daily-question&envId=2026-09-30

class Solution {
    fun maxDepthAfterSplit(seq: String): IntArray {
        var result = IntArray(seq.length)
        var depth = 1

        for (i in 0 until seq.length) {
            if (seq[i] == '(') {
                depth++
                result[i] = depth % 2
            } else {
                result[i] = depth % 2
                depth--
            }
        }

        return result
    }
}
