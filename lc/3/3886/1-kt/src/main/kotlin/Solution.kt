// https://leetcode.com/problems/sum-of-sortable-integers/?envType=daily-question&envId=2026-10-30
import kotlin.math.*

class Solution {
    var nums = intArrayOf()
    var desc = emptyArray<Int>()
    var sparseMax = emptyArray<IntArray>()
    var sparseMin = emptyArray<IntArray>()

    fun sortableIntegers(nums: IntArray): Int {
        val len = nums.size
        this.nums = nums

        // prefix sum of times numbers descend
        desc = Array(len) { 0 }
        var last = nums[0]
        var count = 0
        for ((i, num) in nums.withIndex()) {
            if (num < last) count++
            desc[i] = count
            last = num
        }

        if (desc.last() == 0)
            return solveOrdered(len)

        // each group can handle at most one descend
        val kmax = len / desc.last()
        // length one would not be able to reorder
        val kmin = 2

        // calculate min and max sparse tables
        val levels = floor(log(kmax * 1.0, 2.0)).toInt()
        sparseMax = Array(levels + 1) { nums }
        sparseMin = Array(levels + 1) { nums }
        var half = 1
        for (l in 1..levels) {
            val pad = half * 2 - 1
            sparseMax[l] = IntArray(len - pad) { i ->
                maxOf(sparseMax[l - 1][i], sparseMax[l - 1][i + half])
            }
            sparseMin[l] = IntArray(len - pad) { i ->
                minOf(sparseMin[l - 1][i], sparseMin[l - 1][i + half])
            }
            half *= 2
        }

        var res = 0
        for (k in kmin..kmax)
            if (len % k == 0 && isSortable(k))
                res += k

        return res
    }

    fun isSortable(len: Int): Boolean {
        var left = 0
        var prevMax = 0

        var level = 0
        var sparseLen = 1
        while (sparseLen <= len) {
            level++
            sparseLen *= 2
        }
        level--
        sparseLen /= 2

        while (left < nums.size) {
            val right = left + len - 1

            // current group has at most one descend
            var descCount = desc[right] - desc[left]
            if (nums[right] > nums[left]) descCount++
            if (descCount > 1) return false

            // compare max and min
            val min = minOf(sparseMin[level][left], sparseMin[level][right - sparseLen + 1])
            if (min < prevMax) return false

            prevMax = maxOf(sparseMax[level][left], sparseMax[level][right - sparseLen + 1])
            left += len
        }

        return true
    }

    fun solveOrdered(length: Int): Int {
        var res = 0
        for (k in 1..length)
            if (length % k == 0)
                res += k
        return res
    }
}
