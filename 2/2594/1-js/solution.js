/**
 * @param {number[]} ranks
 * @param {number} cars
 * @return {number}
 */
var repairCars = function(ranks, cars) {
  // greedy assign next car to car mechanic
  // which has lowest time for processing next time

  // keep car mechanics as ordered priority queue

  // change ranks into priority queue of car mechanics
  // [time of processing next car, rank, cars]
  ranks = ranks
    .sort((a,b) => a - b)
    .map(rank => [rank, rank, 0])

  while (cars) {
    cars--
    ranks[0][2]++
    ranks[0][0] = ranks[0][1] * (ranks[0][2] + 1)**2

    // update top and bubble down
    let i = 0
    while (true) {
      let bestIdx = i
      const leftIdx = (i << 1) + 1, rightIdx = (i << 1) + 2
      if (ranks[leftIdx]?.[0] < ranks[bestIdx][0]) { bestIdx = leftIdx }
      if (ranks[rightIdx]?.[0] < ranks[bestIdx][0]) { bestIdx = rightIdx }

      // no changes needed
      if (bestIdx === i) { break }

      // swap
      const temp = ranks[i]
      ranks[i] = ranks[bestIdx]
      ranks[bestIdx] = temp
      i = bestIdx
    }
  }

  // find result by scanning car mechanics
  // after the cars have been greedy assigned
  let result = 0
  for (let i = 0; i < ranks.length; i++) {
    result = Math.max(result, ranks[i][1] * ranks[i][2]**2)
  }
  return result
};

export default repairCars
