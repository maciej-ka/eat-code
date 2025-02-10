<?php

namespace App;

class Solution {
    /**
     * @param Integer[] $nums
     * @param Integer $k
     * @return Integer[]
     */
    function topKFrequent($nums, $k): array {
      $counts = [];
      foreach ($nums as $v) {
        $counts[$v] = array_key_exists($v, $counts) ? $counts[$v] + 1 : 1;
      }
      arsort($counts);
      return array_slice(array_keys($counts), 0, $k);
    }
}

