// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-i/?envType=daily-question&envId=2026-10-15
import java.util.*

class Solution {
    fun sumCounts(nums: List<Int>): Int {
        var res = 0
        val len = nums.size
        var counts = IntArray(101)
        var distinct = 0

        for (i in 0 until len) {
            if (i % 2 == 0) {
                for (k in i until len) {
                    if (++counts[nums[k]] == 1) distinct++
                    res += distinct * distinct
                }
                if (--counts[nums[i]] == 0) distinct--
            } else {
                for (k in len - 1 downTo i) {
                    res += distinct * distinct
                    if (--counts[nums[k]] == 0) distinct--
                }
            }
        }

        return res
    }
}
