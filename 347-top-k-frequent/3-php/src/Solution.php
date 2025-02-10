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
    foreach ($nums as $num) {
      $counts[$num] = array_key_exists($num, $counts) ? $counts[$num] + 1 : 1;
    }

    // to speed up sorting, bucket same count values
    $buckets = [];
    foreach ($counts as $num => $count) {
      $buckets[$count] = array_key_exists($count, $buckets)
        ? array_merge($buckets[$count], [$num])
        : [$num];
    }

    krsort($buckets);

    $result = [];
    foreach($buckets as $bucket) {
      $slice = array_slice($bucket, 0, $k - count($result));
      $result = array_merge($result, $slice);
      if (count($result) >= $k) return $result;
    }
  }
}

