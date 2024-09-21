package main

import (
  "testing"
  "reflect"
)

func TestSeq(t *testing.T) {
  got := Seq(4)
  want := []int{0,1,2,3}
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
}

func TestPull(t *testing.T) {
  arr := []int{0,1,2,3}
  got := Pull(3, 0, arr)
  want := []int{3,0,1,2}
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
  if !reflect.DeepEqual(want, arr) {
    t.Error("expected", want, "got", arr)
  }
}

func TestCalc(t *testing.T) {
  got := Calc([]int{2}, []int{4})
  want := 2
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }

  got = Calc([]int{2,3,4}, []int{0,1,2})
  want = 0
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }

  got = Calc([]int{2,3,8}, []int{0,6,7})
  want = 1
  if !reflect.DeepEqual(want, got) {
    t.Error("expected", want, "got", got)
  }
}

func TestCalcScores(t *testing.T) {
  step := Step{
    pos: 0,
    perm: []int{0,1,2,3},
    appl: []int{3,0,1,2},
    scores: []int{0,0,0,0},
  }
  CalcScores(step)
  want := []int{0,0,0,0}
  if !reflect.DeepEqual(want, step.scores) {
    t.Error("expected", want, "got", step.scores)
  }

  step = Step{
    pos: 1,
    perm: []int{0,2,1,3},
    appl: []int{3,0,1,2},
    scores: []int{0,0,0,0},
  }
  CalcScores(step)
  want = []int{0,0,0,0}
  if !reflect.DeepEqual(want, step.scores) {
    t.Error("expected", want, "got", step.scores)
  }

  step = Step{
    pos: 2,
    perm: []int{0,2,3,1},
    appl: []int{3,0,2,1},
    scores: []int{0,0,0,0},
  }
  CalcScores(step)
  want = []int{0,0,2,2}
  if !reflect.DeepEqual(want, step.scores) {
    t.Error("expected", want, "got", step.scores)
  }

  step = Step{
    pos: 1,
    perm: []int{0,1,2,3},
    appl: []int{3,1,0,2},
    scores: []int{0,0,0,0},
  }
  CalcScores(step)
  want = []int{1,1,0,0}
  if !reflect.DeepEqual(want, step.scores) {
    t.Error("expected", want, "got", step.scores)
  }

  step = Step{
    pos: 1,
    perm: []int{3,1,0,2,4},
    appl: []int{3,0,1,2,4},
    scores: []int{0,0,0,0,0},
  }
  CalcScores(step)
  want = []int{3,0,0,0,1}
  if !reflect.DeepEqual(want, step.scores) {
    t.Error("expected", want, "got", step.scores)
  }

  step = Step{
    pos: 1,
    perm: []int{1,0,2,3},
    appl: []int{3,2,0,1},
    scores: []int{0,0,0,0},
  }
  CalcScores(step)
  want = []int{1,0,1,0}
  if !reflect.DeepEqual(want, step.scores) {
    t.Error("expected", want, "got", step.scores)
  }
}
