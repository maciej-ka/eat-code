// https://leetcode.com/problems/count-connected-components-in-lcm-graph/submissions/2170001576/
import java.util.*

class Solution {
    lateinit var groups: IntArray

    fun countComponents(nums: IntArray, threshold: Int): Int {
        val max = minOf(threshold, 200000)
        groups = IntArray(max + 1)
        var res = 0

        for (base in nums) {
            if (base > threshold) {
                res++
                continue
            }

            if (groups[base] != 0)
                continue

            groups[base] = base
            res++

            var group = base
            var num = base + base
            while (num <= max) {
                if (groups[num] == 0)
                    groups[num] = group
                else {
                    val another = find(groups[num])
                    if (group != another) {
                        groups[group] = another
                        group = another
                        res--
                    }
                }

                num += base
            }
        }

        return res
    }

    fun find(num: Int): Int {
        if (groups[num] != num)
            groups[num] = find(groups[num])
        return groups[num]
    }
}
