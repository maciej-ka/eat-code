#!/usr/bin/env bats

setup() {
  source solution.sh
}

@test "test 1" {
  arr=(1 2 3)
  result=$(solve "${arr[@]}")
  [ "$result" -eq 3 ]
}
