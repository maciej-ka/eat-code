package solution

import (
  "testing"
  "reflect"
)

func Test1(t *testing.T) {
  got := FindMissingAndRepeatedValues([][]int{{1,3},{2,2}})
  want := []int{2,4}
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
}

func Test2(t *testing.T) {
  got := FindMissingAndRepeatedValues([][]int{{9,1,7},{8,9,2},{3,4,6}})
  want := []int{9,5}
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
}
