// https://leetcode.com/problems/minimum-operations-to-sort-a-string/submissions/2177803529/?envType=daily-question&envId=2026-10-21
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
            when {
                char > max -> { max = char; maxCount = 1 }
                char == max -> maxCount++
            }
            when {
                char < min -> { min = char; minCount = 1 }
                char == min -> minCount++
            }
            sorted = sorted && char >= last
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
