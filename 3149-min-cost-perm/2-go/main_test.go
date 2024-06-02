package main

import (
  "testing"
  "reflect"
)

func TestSeq(t *testing.T) {
  got := Seq(4)
  want := []int{0,1,2,3}
  if !reflect.DeepEqual(got, want) {
    t.Error("expected", want, "got", got)
  }
}

func TestPull(t *testing.T) {
  arr := []int{0,1,2,3}
  got := Pull(3, 0, arr)
  want := []int{3,0,1,2}
  if !reflect.DeepEqual(got, want) {
    t.Error("expected", want, "got", got)
  }
  if !reflect.DeepEqual(arr, want) {
    t.Error("expected", want, "got", arr)
  }
}

func TestCalc(t *testing.T) {
  got := Calc([]int{2}, []int{4})
  want := 2
  if !reflect.DeepEqual(got, want) {
    t.Error("expected", want, "got", got)
  }

  got = Calc([]int{2,3,4}, []int{0,1,2})
  want = 0
  if !reflect.DeepEqual(got, want) {
    t.Error("expected", want, "got", got)
  }

  got = Calc([]int{2,3,8}, []int{0,6,7})
  want = 1
  if !reflect.DeepEqual(got, want) {
    t.Error("expected", want, "got", got)
  }
}

