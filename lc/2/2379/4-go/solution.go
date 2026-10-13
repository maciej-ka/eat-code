package solution

func MinimumRecolors(blocks string, k int) int {
  count := 0
  for i := range k {
    if (blocks[i] == 'W') { count++ }
  }

  best := count
  endIdx := len(blocks) - k
  for i := range endIdx {
    if (blocks[i] == 'W') { count-- }
    if (blocks[i + k] == 'W') { count++ }
    if (count < best) { best = count }
  }

  return best
}

