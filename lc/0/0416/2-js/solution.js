/**
 * @param {number[]} nums
 * @return {boolean}
 */

// not possible if:
// total sum is uneven
// (or uneven have uneven count)

// not possible if:
// elements left ahead are smaller
// then difference between sets

// idea A: "EQUILIBRUM"
// Keep diff between sets smallest.
// Always add next element
// to smaller of two sums (sumA, sumB).
// Process largest nums first.

// if fails, increase count of "alterations"
// aka decisions to make differently

var canPartition = function(nums) {
  // process numbers in descending order
  nums.sort((a, b) => b - a)

  // total count
  let total = 0
  for (const num of nums) { total += num }

  // uneven
  if (total % 2) { return false }

  // count sums
  const sums = new Uint16Array(2)

  // increasing number of alterations
  for (let alt = 0; alt < nums.length; alt++) {
    // reset run
    sums.fill(0)
    let ahead = total;
    for (let i = 0; i < nums.length; i++) {
      sums[0] < sums[1] && alt < i ? sums[0] += nums[i] : sums[1] += nums[i]
      ahead -= nums[i]
      const diff = Math.abs(sums[0] - sums[1])
      // success: can place rest into one set to solve
      if (diff === ahead) { return true }
      // abort: difference larger then rest of elements
      if (diff > ahead) { continue }
    }
  }

  // each alteration run failed
  return false
};

export default canPartition
