// https://leetcode.com/problems/sum-of-squares-of-special-elements/submissions/2169309742/?envType=daily-question&envId=2026-10-11
import java.util.*

class Solution {
    fun sumOfSquares(nums: IntArray): Int {
        var res = 0
        var len = nums.size
        for (i in 0..len)
            if (len % (i + 1) == 0)
                res += nums[i] * nums[i]

        return res
    }
}
