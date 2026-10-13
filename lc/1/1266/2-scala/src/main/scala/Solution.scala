// https://leetcode.com/problems/minimum-time-visiting-all-points/?envType=daily-question&envId=2026-01-12

object Solution {
  def minTimeToVisitAllPoints(points: Array[Array[Int]]): Int = {
    if points.length == 1 then return 0
    var sum = 0
    for
      pair <- points.sliding(2)
    do
      sum += math.max(
        math.abs(pair(0)(0) - pair(1)(0)),
        math.abs(pair(0)(1) - pair(1)(1)))
    sum
  }
}
