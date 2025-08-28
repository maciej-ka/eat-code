package solution

import (
  "testing"
  "reflect"
)

func Test1(t *testing.T) {
  actual := solve([]int{1, 2, 3})
  expected := 3
  if !reflect.DeepEqual(expected, actual) {
    t.Error("expected", expected, "actual", actual)
  }
}
