// https://leetcode.com/problems/replace-non-coprime-numbers-in-array/submissions/1773274995
import _ from 'lodash';
import {} from 'datastructures-js';

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
  const result = [nums[0]]
  let i = 1

  while (true) {
    if (result.length === 1) {
      if (i >= nums.length) return result;
      result.push(nums[i++]);
    }

    const a = result[result.length - 2];
    const b = result[result.length - 1];
    const gcd = calcGcd(a, b);

    // non-coprime
    if (gcd !== 1) {
      result.pop();
      result[result.length - 1] = (a / gcd) * b;
      continue;
    }

    // coprime
    if (i >= nums.length) return result;
    result.push(nums[i++]);
  }
}

export default replaceNonCoprimes;
