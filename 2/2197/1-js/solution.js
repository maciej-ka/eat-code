// https://leetcode.com/problems/replace-non-coprime-numbers-in-array/submissions/1773252251
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
    const len = result.length

    if (len === 1) {
      if (i >= nums.length) return result;
      result.push(nums[i++]);
    }

    const b = result.pop();
    const a = result.pop();
    const gcd = calcGcd(a, b);

    // non-coprime
    if (gcd !== 1) {
      result.push((a / gcd) * (b / gcd) * gcd);
      continue;
    }

    // coprime
    result.push(a)
    result.push(b)
    if (i >= nums.length) return result;
    result.push(nums[i++]);
  }
}

export default replaceNonCoprimes;
