#!/usr/bin/env bats

setup() {
  source solution.sh
}

@test "solve returns correct length for array [1, 2, 3]" {
  arr=(1 2 3)
  result=$(solve "${arr[@]}")
  [ "$result" -eq 3 ]
}
