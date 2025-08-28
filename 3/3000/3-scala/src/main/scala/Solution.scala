// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/?envType=daily-question&envId=2025-08-26
// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/submissions/1748933569/?envType=daily-question&envId=2025-08-26

object Solution {
    def areaOfMaxDiagonal(dimensions: Array[Array[Int]]): Int = {
        // square of diagonal
        var maxScore = 0
        var maxArea = 0
        for
            pair <- dimensions
            score = pair(0) * pair(0) + pair(1) * pair(1)
            if score >= maxScore
        do
            val area = pair(0) * pair(1)
            if score > maxScore then
                maxScore = score
                maxArea = area
            else if area > maxArea then
                maxArea = area
        maxArea
    }
}
