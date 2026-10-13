package solution

func FindMissingAndRepeatedValues(grid [][]int) []int {
  counts := make([]int, len(grid) * len(grid) + 1)
  for i := range grid {
    for _, value := range grid[i] {
      counts[value]++
    }
  }

  var num1, num2 int
  for value, count := range counts {
    if (count == 2) { num1 = value }
    if (count == 0) { num2 = value }
  }
  return []int{num1, num2}
}
