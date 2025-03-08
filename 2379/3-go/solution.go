package solution

func MinimumRecolors(blocks string, k int) int {
  count := 0
  for i := 0; i < k; i++ {
    if (blocks[i] == 'W') { count++ }
  }

  best := count
  endIdx := len(blocks) - k
  for i := 0; i < endIdx; i++ {
    if (blocks[i] == 'W') { count-- }
    if (blocks[i + k] == 'W') { count++ }
    if (count < best) { best = count }
  }

  return best
}
