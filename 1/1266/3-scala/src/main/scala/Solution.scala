// https://leetcode.com/problems/minimum-time-visiting-all-points/submissions/1883177720/?envType=daily-question&envId=2026-01-12

object Solution {
  def minTimeToVisitAllPoints(points: Array[Array[Int]]): Int = {
    points.sliding(2).collect {
      case Array(Array(x1, y1), Array(x2, y2)) =>
        math.max(math.abs(x2 - x1), math.abs(y2 - y1))
    }.sum
  }
}
