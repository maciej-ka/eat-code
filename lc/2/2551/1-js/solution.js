/**
 * @param {number[]} weights
 * @param {number} k
 * @return {number}
 */

// no bag is empty
// divide subarrays betwen bags
// cost is a sum of ends weight

// bag cannot have two subarrays

// task is about creating k - 1 divisions
// every division decides on two values

// first and last element
// will always contribute to score

// score is sum of bags
// bags are sum of edges
// so score is sum of edges

var putMarbles = function(weights, k) {
  // sum for each possible division point
  // divs[0] is division between 0 and 1
  const divs = new Uint32Array(weights.length - 1)
  for (let i = 0; i < weights.length - 1; i++) {
    divs[i] = weights[i] + weights[i + 1]
  }

  // sort descending
  divs.sort((a, b) => b - a)
  console.log(divs);

  // to have k bags, we need to divide k - 1
  // result is difference between k -1 lowest and highest
  let result = 0
  for (let i = 0; i < k - 1; i++) {
    result += divs[i] - divs[divs.length - 1 - i]
  }
  return result
};

export default putMarbles

// example
// 1,3,5,1
//
// divs
// 4,8,6
