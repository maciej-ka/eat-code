// https://leetcode.com/problems/minimum-operations-to-sort-a-permutation/submissions/2179726894/?envType=daily-question&envId=2026-10-23
import java.util.*

class Solution {
    fun minOperations(nums: IntArray): Int {
        var isAsc = true
        var izero = 0
        var last = 0

        // check is sorted
        // a) check is ascending
        last = nums.last()
        for (i in nums.indices) {
            if (nums[i] == 0)
                izero = i
            else if (nums[i] != last + 1) {
                isAsc = false
                break
            }
            last = nums[i]
        }

        // b) check is descending
        if (!isAsc) {
            last = nums[0]
            for (k in nums.size - 1 downTo 0) {
                if (nums[k] == 0)
                    izero = k
                else if (nums[k] != last + 1)
                    return -1
                last = nums[k]
            }
        }

        return if (isAsc)
            minOf(izero, nums.size - izero + 2)
        else
            minOf(izero + 2, nums.size - izero)
    }
}
