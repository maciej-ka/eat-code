/**
 * @param {string} s
 * @param {number} numOps
 * @return {number}
 */
var minLength = function (s, numOps) {
  // count[2]: how many sequences with length 2
  const counts = new Map();
  let digit = s[0];
  let length = 0;

  // solution 1, variants starting with 0 and 1
  let one0 = 0;
  let one1 = 0;

  // count lengths
  for (let i = 0; i < s.length; i++) {
    if (digit !== s[i]) {
      counts.set(length, (counts.get(length) || 0) + 1);
      length = 0;
      digit = s[i];
    }
    ~~s[i] === i % 2 ? one1++ : one0++;
    length++;
  }
  counts.set(length, (counts.get(length) || 0) + 1);

  // check is solution 1 possible
  if (one0 <= numOps || one1 <= numOps) {
    return 1;
  }

  // export hash keys
  const lengths = 
    Array.from(counts.keys())
    .map(x => ~~x)
    .sort((a, b) => b - a);

  // binary search lowest possible result
  let low = 2;
  let high = lengths[0];
  let m;
  while (low < high) {
    m = (low + high) >> 1
    isSolutionPossible(m, lengths, counts, numOps) ? (high = m) : (low = m + 1);
  }

  return high;
};

function isSolutionPossible(goal, lengths, counts, numOps) {
  let sum = 0;
  // slice too long sequences to fit the goal
  for (let i = 0; i < lengths.length && lengths[i] > goal; i++) {
    const multiplier = Math.floor(lengths[i] / (goal + 1));
    sum += counts.get(lengths[i]) * multiplier;

    // number or moves already too high, early exit
    if (sum > numOps) {
      return false;
    }
  }
  return true;
}

export default minLength;
