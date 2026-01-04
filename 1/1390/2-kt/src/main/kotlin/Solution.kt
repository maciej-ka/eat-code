// https://leetcode.com/problems/four-divisors/submissions/1874282787

import kotlin.math.sqrt

class Solution {
    fun sumFourDivisors(nums: IntArray): Int {
        var res = 0

        nums@ for (n in nums) {
            var sum = 1 + n
            var count = 2
            val root = sqrt(n.toDouble()).toInt()
            for (d in 2..root) {
                if (n % d != 0) continue
                sum += d
                count++
                val div = n / d
                if (d != div) {
                    sum += div
                    count++
                }
                if (count > 4) continue@nums
            }
            if (count == 4) res += sum
        }

        return res
    }
}
