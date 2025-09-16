// https://leetcode.com/problems/replace-non-coprime-numbers-in-array/submissions/1773276616
import _ from 'lodash';
import { Stack } from 'datastructures-js';

function calcGcd(arg1, arg2) {
  let a, b, r;
  if (arg1 > arg2) {
    a = arg1;
    b = arg2;
  } else {
    a = arg2;
    b = arg1;
  }

  r = a % b;
  while (r) {
    a = b;
    b = r;
    r = a % b;
  }
  return b;
}

/**
 * @param {number[]} nums
 * @return {number[]}
 */
var replaceNonCoprimes = function(nums) {
  const result = new Stack([nums[0]])
  let i = 1

  while (true) {
    if (result.size() === 1) {
      if (i >= nums.length) return result.toArray();
      result.push(nums[i++]);
    }

    const b = result.pop();
    const a = result.pop();
    const gcd = calcGcd(a, b);

    // non-coprime
    if (gcd !== 1) {
      result.push((a / gcd) * b);
      continue;
    }

    // coprime
    result.push(a)
    result.push(b)
    if (i >= nums.length) return result.toArray();
    result.push(nums[i++]);
  }
}

export default replaceNonCoprimes;
