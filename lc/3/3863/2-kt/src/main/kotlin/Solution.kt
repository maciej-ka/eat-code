// https://leetcode.com/problems/minimum-operations-to-sort-a-string/submissions/2177811255/?envType=daily-question&envId=2026-10-21
import java.util.*

class Solution {
    fun minOperations(s: String): Int {
        var sorted = true
        var last = s[0]
        var max = 'a'
        var min = 'z'
        var maxCount = 0
        var minCount = 0

        for (char in s) {
            if (char > max) { max = char; maxCount = 0 }
            if (char < min) { min = char; minCount = 0 }
            if (char == max) maxCount++
            if (char == min) minCount++
            if (char < last) sorted = false
            last = char
        }

        return when {
            sorted -> 0
            s.length == 2 -> -1
            s[0] == max && s.last() == min && maxCount == 1 && minCount == 1 -> 3
            s[0] == min || s.last() == max -> 1
            else -> 2
        }
    }
}
