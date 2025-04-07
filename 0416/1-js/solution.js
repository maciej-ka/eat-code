/**
 * @param {number[]} nums
 * @return {boolean}
 */

// not possible if:
// total sum is uneven
// (or uneven have uneven count)

// idea A: "EQUILIBRUM"
// Keep diff between sets smallest.
// Always add next element
// to smaller of two sums (sumA, sumB).
// Process largest nums first.

// idea B: "REST STEALING"
// Fill one as long as rest allows.
// To to fill setA until setA === rest
// if not possible, then remember difference
// and start filling setB to a point where
// setB === rest + difference.

// idea C: "META DESPAIR"
// try both A and B
// fail only if both fail

var canPartition = function(nums) {
  // process numbers in descending order
  nums.sort((a, b) => b - a)

  // total ahead
  let ahead = 0
  for (const num of nums) { ahead += num }

  // uneven
  if (ahead % 2) { return false }

  // count sums
  const sums = new Uint16Array(2)
  let active = 0

  // EQUILIBRUM
  // try to solve by adding to smallest
  for (const num of nums) {
    sums[0] < sums[1] ? sums[0] += num : sums[1] += num
  }
  if (sums[0] === sums[1]) { return true }

  // REST STEALING
  // goal where we try to meet
  // active sum === ahead + goal
  sums.fill(0)
  let goal = 0

  for (let i = 0; i < nums.length; i++) {
    sums[active] += nums[i]
    ahead -= nums[i]

    // try to meet sums[active] === ahead + goal
    const diff = ahead + goal - sums[active]
    // put all the rest into second set to have result
    if(diff === 0) { return true }

    // change active
    if (nums[i + 1] << 1 > diff) {
      goal = Math.abs(sums[0], sums[1])
      active = active + 1 % 2
    }
  }

  // both ideas failed
  return false
};

export default canPartition
