// https://leetcode.com/problems/count-days-spent-together/submissions/2180452934/?envType=daily-question&envId=2026-10-24
import java.util.*

class Solution {
    val months = intArrayOf(31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31)

    fun countDaysTogether(arriveAlice: String, leaveAlice: String, arriveBob: String, leaveBob: String): Int {
        val arrive = maxOf(dayNumber(arriveAlice), dayNumber(arriveBob))
        val leave = minOf(dayNumber(leaveAlice), dayNumber(leaveBob))
        return maxOf(0, leave - arrive + 1)
    }

    fun dayNumber(date: String): Int {
        var result = 0
        val month = date.substring(0, 2).toInt()
        for (i in 0 until month - 1) result += months[i]
        return result + date.substring(3).toInt()
    }
}
