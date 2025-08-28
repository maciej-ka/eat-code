package solution

import (
  "testing"
  "reflect"
)

func Test1(t *testing.T) {
  got := MinimumRecolors("WBBWWBBWBW", 7)
  want := 3
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
}

func Test2(t *testing.T) {
  got := MinimumRecolors("WBWBBBW", 2)
  want := 0
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
}

func Test3(t *testing.T) {
  got := MinimumRecolors("BWWWBB", 6)
  want := 3
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
}
