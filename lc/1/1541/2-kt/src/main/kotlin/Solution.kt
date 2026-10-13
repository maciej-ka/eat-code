// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/submissions/2167082957/?envType=daily-question&envId=2026-10-09
import java.util.*

class Solution {
    fun minInsertions(s: String): Int {
        var res = 0
        var opened = 0
        var i = 0

        while (i < s.length) {
            if (s[i] == '(') opened++
            else { // )
                if (opened == 0) res++
                else opened--

                if (i + 1 == s.length) res++
                else
                  if (s[i + 1] == ')') i++
                  else res++
            }
            i++
        }

        return res + opened * 2
    }
}
