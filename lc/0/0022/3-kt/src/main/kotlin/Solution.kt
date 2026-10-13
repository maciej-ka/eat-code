// thttps://leetcode.com/problems/generate-parentheses/submissions/2160545917/?envType=daily-question&envId=2026-10-02
class Solution {

    private val result = ArrayList<String>();

    fun generateParenthesis(n: Int): List<String> {
        build("", 0, 0, n);
        return result
    }

    fun build(s: String, open: Int, closed: Int, n: Int) {
        if (open == n && closed == n) {
            result.add(s)
            return
        }

        if (open < n)
            build(s + "(", open + 1, closed, n);
        if (closed < open)
            build(s + ")", open, closed + 1, n);
    }
}
