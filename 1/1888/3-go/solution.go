// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/submissions/1941239955/?envType=daily-question&envId=2026-03-07

package solution

func minFlips(s string) int {
  n := len(s)
  c0 := 0
  c1 := 0
  s0 := make([]int, n)
  s1 := make([]int, n)

  for i := range(n) {
    if ((s[i] == '0') == (i % 2 == 0)) {
      c0++
    } else {
      c1++
    }
    s0[i] = c0
    s1[i] = c1
  }

  last0 := s0[n - 1]
  last1 := s1[n - 1]
  best := min(last0, last1)
  if (n % 2 == 0) { return best }

  for i := range(n) {
    best = min(best, s0[i] + last1 - s1[i])
    best = min(best, s1[i] + last0 - s0[i])
  }
  return best
}

