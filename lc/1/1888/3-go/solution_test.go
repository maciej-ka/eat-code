package solution

import (
  "testing"
  "reflect"
)

func Test1(t *testing.T) {
  actual := minFlips("111000")
  expected := 2
  if !reflect.DeepEqual(expected, actual) {
    t.Error("expected", expected, "actual", actual)
  }
}

func Test2(t *testing.T) {
  actual := minFlips("010")
  expected := 0
  if !reflect.DeepEqual(expected, actual) {
    t.Error("expected", expected, "actual", actual)
  }
}

func Test3(t *testing.T) {
  actual := minFlips("1110")
  expected := 1
  if !reflect.DeepEqual(expected, actual) {
    t.Error("expected", expected, "actual", actual)
  }
}

func Test4(t *testing.T) {
  actual := minFlips("101001010")
  expected := 0
  if !reflect.DeepEqual(expected, actual) {
    t.Error("expected", expected, "actual", actual)
  }
}

func Test5(t *testing.T) {
  actual := minFlips("101100101")
  expected := 2
  if !reflect.DeepEqual(expected, actual) {
    t.Error("expected", expected, "actual", actual)
  }
}
