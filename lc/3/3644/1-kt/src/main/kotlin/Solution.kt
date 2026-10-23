// https://leetcode.com/problems/maximum-k-to-sort-a-permutation/submissions/2180031933/?envType=daily-question&envId=2026-10-23
import java.util.*

class Solution {
    fun sortPermutation(nums: IntArray): Int {
        var res = 0
        var empty = true

        for (i in nums.indices)
            if (nums[i] != i) {
                if (nums[i] == 0) return 0
                if (empty) {
                    res = nums[i]
                    empty = false
                } else
                    res = res and nums[i]
            }

        return res
    }
}
