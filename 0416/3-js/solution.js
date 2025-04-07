/**
 * @param {number[]} nums
 * @return {boolean}
 */

// count total

// if total is odd: partition not possible

// target sum for each set is known
// total / 2

// first element need to be a part of
// of at least one set, and this will be the set
// we will try to recreate (hoping that it's simpler)

// so problem can be rephrased to:
// pick from nums[1 ... n]
// that will have sum
// (total / 2) - n[0]

// canPick(from, target)
// = canPick(from + 1, target - nums[from])
// || canPick(from + 1, target)
// (variant for take and skip)

// potentially 2**n path (n is up to 2000)
// to limit number, early quit:
// if sum ahead is not enough
// if sum ahead is exactly what's needed

var canPartition = function(nums) {
  // process numbers in descending order
  nums.sort((a, b) => b - a)

  // walking sum ahead
  let total = new Uint16Array(nums.length)
  for (let i = nums.length - 1; i >= 0; i--) {
    total[i] = nums[i] + (total[i + 1] || 0);
  }

  // check is total even (total is sums[0])
  if (total[0] % 2) { return false }

  // return: is it possible to reach sum
  // by picking elements from nums[i..]
  function canPick(i, sum) {
    if (sum < 0 || i === nums.length || sum > total[i]) { return false }
    if (sum === 0 || sum === total[i]) { return true }
    return canPick(i + 1, sum - nums[i])
      || canPick(i + 1, sum)
  }

  // always pick first element
  const goal = (total[0] >> 1) - nums[0]
  return !goal || canPick(1, goal)
};

export default canPartition
