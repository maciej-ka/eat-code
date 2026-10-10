// https://leetcode.com/problems/minimum-sum-of-squared-difference/submissions/2168412913/?envType=daily-question&envId=2026-10-10
import java.util.*
import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min

class Solution {
    fun minSumSquareDiff(nums1: IntArray, nums2: IntArray, k1: Int, k2: Int): Long {
        var counts = IntArray(100001)
        var maxDiff = 0

        for (i in 0..<nums1.size) {
            val diff = abs(nums1[i] - nums2[i])
            counts[diff]++
            maxDiff = max(maxDiff, diff)
        }

        var have = k1 + k2
        for (i in maxDiff downTo 1) {
            val take = min(counts[i], have)
            counts[i] -= take
            counts[i - 1] += take
            have -= take
            if (have == 0) break
        }

        var result = 0L
        for (i in 0..maxDiff) {
            result += counts[i].toLong() * i * i
        }

        return result
    }
}
