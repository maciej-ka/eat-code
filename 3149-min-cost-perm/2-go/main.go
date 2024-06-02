package main

import "fmt"

type step struct {
  perm []int
  appl []int
  pos int
}

func seq(length int) []int {
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

func findPermutation(nums []int) []int {
  // create zero step
  root := step{
    perm: seq(len(nums)),
    appl: Pull(nums[0], 0, seq(len(nums))),
    pos: 0,
  }
  fmt.Println(root)

  return nums
}

func main() {
  findPermutation([]int{3,1,0,2})
}



// findPermutation([]int{3,1,0,2})
// fmt.Println(len(nums))
// n := len(nums)
// perm := [n]int
// for i := range nums {perm[i] = i}
// fmt.Println(solution{perm: perm, i:0})

// type step struct {
//   // score
//   // children
// }

