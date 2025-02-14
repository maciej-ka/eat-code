/**
 * @param {string} s
 * @param {number} numOps
 * @return {number}
 */
var minLength = function (s, numOps) {
  let ranking = new Array(numOps + 1).fill(0);

  let char = s[0]
  let len = 1
  for (let i=1; i<s.length; i++) {
    if (char !== s[i]) {
      ranking = rank(ranking, len)
      char = s[i]
      len = 0
    }
    len++
  }
  ranking = rank(ranking, len)

  for(let i=0; i<numOps; i++) {
    const max = ranking.shift()
    if (max === 1) { return 1 }
    const a = Math.floor((max - 1) / 2)
    const b = Math.ceil((max - 1) / 2)
    ranking = rank(ranking, a)
    ranking = rank(ranking, b)
  }

  return ranking[0]
};

function rank(ranking, value) {
  // early return if value is too low
  let high = ranking.length - 1
  if (value <= ranking[high]) {
    return ranking
  }

  // binary search value position
  let low = 0
  while(low !== high) {
    const m = high - Math.ceil((high - low) / 2)
    if (value > ranking[m]) {
      high = m
    } else {
      low = m + 1
    }
  }

  // construct new ranking
  return [
    ...ranking.slice(0, low),
    value,
    ...ranking.slice(low, -1)
  ]
}

export default minLength;

