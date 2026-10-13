// https://leetcode.com/problems/maximum-alternating-sum-of-squares/submissions/2171050846/?envType=daily-question&envId=2026-10-13
import java.util.*
import kotlin.math.abs

class Solution {
    fun maxAlternatingSum(nums: IntArray): Long {
        val len = nums.size
        for (i in 0 until len)
            nums[i] *= nums[i]
        nums.sort()

        var res = 0L
        val half = len / 2
        for (i in 0 until half)
            res += nums[half + i] - nums[i]
        if (len % 2 == 1)
            res += nums[len - 1]

        return res
    }
}
