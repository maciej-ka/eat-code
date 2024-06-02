package main

import (
  "testing"
  "reflect"
)

func TestPull(t *testing.T) {
  got := Pull(3, 0, []int{0,1,2,3})
  want := []int{3,0,1,2}
  if !reflect.DeepEqual(got, want) {
    t.Error("expected", want, "got", got)
  }
}
