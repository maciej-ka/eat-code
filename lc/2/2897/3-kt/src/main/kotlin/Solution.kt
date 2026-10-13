// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares/submissions/2170588632/?envType=daily-question&envId=2026-10-12
import java.util.*

class Solution {
    val BITS = 30
    val MOD = 1000000007

    fun maxSum(nums: List<Int>, k: Int): Int {
        var ones = IntArray(BITS)
        for (num in nums)
            for (i in 0 until BITS)
                ones[i] += (num shr i) and 1

        var res = 0L
        for (j in 1..k) {
            var n = 0L
            for (i in 0 until BITS)
                if (ones[i] > 0) {
                    n = n or (1L shl i)
                    ones[i]--
                }
            res = (res + n * n) % MOD
        }
        return res.toInt()
    }
}
