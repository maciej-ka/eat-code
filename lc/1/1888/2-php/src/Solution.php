<?php
// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/submissions/1941186936/?envType=daily-question&envId=2026-03-07

namespace App;

class Solution {

  /**
   * @param String $s
   * @return Integer
   */
  function minFlips($s) {
    $n = strlen($s);
    $c0 = 0;
    $c1 = 0;
    $s0 = [];
    $s1 = [];

    for ($i = 0; $i < $n; $i++) {
      if (($i % 2 == 0) == ($s[$i] == '0')) {
        $c0++;
      } else {
        $c1++;
      }
      array_push($s0, $c0);
      array_push($s1, $c1);
    }

    $last0 = $s0[$n - 1];
    $last1 = $s1[$n - 1];
    $best = min($last0, $last1);
    if ($n % 2 == 0) { return $best; }

    for ($i = 0; $i < $n; $i++) {
      $best = min($best, $s0[$i] + $last1 - $s1[$i]);
      $best = min($best, $s1[$i] + $last0 - $s0[$i]);
    }

    return $best;
  }
}
