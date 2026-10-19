// https://leetcode.com/problems/find-target-indices-after-sorting-array/submissions/2176111223/?envType=daily-question&envId=2026-10-19
import java.util.*

class Solution {
    fun targetIndices(nums: IntArray, target: Int): List<Int> {
        var count = 0
        var smaller = 0
        for (num in nums)
            if (num < target)
                smaller++
            else if (num == target)
                count++

        var res = mutableListOf<Int>()
        for (i in 0 until count)
            res.add(smaller + i)

        return res
    }
}
