package main

import (
  "fmt"
)

type Step struct {
  pos int
  perm []int
  appl []int
  scores []int
}

func Seq(length int) []int {
  result := make([]int, length)
  for i := range result { result[i] = i }
  return result
}

func Pull(value int, pos int, arr []int) []int {
  for i := len(arr)-1; i > pos; i-- {
    if arr[i] == value {
      arr[i-1], arr[i] = arr[i], arr[i-1]
    }
  }
  return arr
}

func Calc(a, b []int) int {
  ai, bi, best := 0, 0, -1
  var diff int
  for ai < len(a) && bi < len(b) {
    diff = a[ai] - b[bi]
    if diff < 0 { diff = -diff }
    if diff == 0 { return 0 }
    if diff < best || best == -1 { best = diff }
    if a[ai] > b[bi] { bi++ } else { ai++ }
  }
  return best
}

func CalcScores(s Step) {
  var a, b []int
  bi := 1
  for i := range s.scores {
    if i <= s.pos { a = s.perm[i:i+1] } else { a = s.perm[s.pos+1:] }
    if bi <= s.pos { b = s.appl[bi:bi+1]} else { b = s.appl[s.pos+1:] }
    s.scores[i] = Calc(a, b)
    bi = (bi + 1) % len(s.scores)
  }
}

func findPermutation(nums []int) []int {
  // create zero step
  root := Step {
    pos: 0,
    perm: Seq(len(nums)),
    appl: Pull(nums[0], 0, Seq(len(nums))),
    scores: make([]int, len(nums)),
  }
  fmt.Println(root)
  return nums
}

func main() {
  findPermutation([]int{3,1,0,2})
}

// type Step struct {
//   // scores
//   // total
//   // children
// }

