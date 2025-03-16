import { expect, test } from 'vitest'
import solution from "./solution";

/**
 * greedy: give the next car
 * to the fastest current mechanic
 *
 * r * n**2
 * let's make a priority queue with a top element
 * being fastest from currently available
 *
 * each cell with time to process next car
 * [1 <= 1*(1*1), 2 <= 2*(1*1), 3 <= 3*(1*1), 4 <= 4*(1*1), ]
 *
 * give first car to 1
 * update mechanic
 * [4 <= 1*(2*2)]
 *
 * update priority queue
 * [2 <= 2*(1*1), 3 <= 3*(1*1), 4 <= 4*(1*1), 4 <= 1*(2*2), ]
 *
 * give 2nd car to 2
 * [3 <= 3*(1*1), 4 <= 4*(1*1), 4 <= 1*(2*2), 8 <= 2*(2*2), ]
 *
 * give 3rd car ... up to 10
 * [4 <= 4*(1*1), 4 <= 1*(2*2), 8 <= 2*(2*2), 12 <= 3*(2*2), ]
 * [4 <= 1*(2*2), 8 <= 2*(2*2), 12 <= 3*(2*2), 16 <= 4*(2*2), ]
 * [8 <= 2*(2*2), 9 <= 1*(3*3), 12 <= 3*(2*2), 16 <= 4*(2*2), ]
 * [9 <= 1*(3*3), 12 <= 3*(2*2), 16 <= 4*(2*2), 18 <= 2*(3*3), ]
 * [12 <= 3*(2*2), 16 <= 4*(2*2), 16 <= 1*(4*4), 18 <= 2*(3*3), ]
 * [16 <= 4*(2*2), 16 <= 1*(4*4), 18 <= 2*(3*3), 27 <= 3*(3*3), ]
 * [16 <= 1*(4*4), 18 <= 2*(3*3), 27 <= 3*(3*3), 36 <= 4*(3*3), ]
 * [18 <= 2*(3*3), 27 <= 3*(3*3), 36 <= 4*(3*3), 25 <= 1*(5*5), ]
 *
 * because current values are what would be the cost of processing next car
 * then actuall number of cars per mechanic is lower by one
 *
 * to get actual time, decrease each by one car
 * [8 <= 2*(2*2), 12 <= 3*(2*2), 16 <= 4*(2*2), 16 <= 1*(4*4), ]
 *
 * result is max
 * [8, 12, 16, 16]
 *
 * result is 16
 *
 */
test('case 1', () => {
  const actual = solution([4,2,3,1], 10);
  expect(actual).toEqual(16);
});

/**
 * start
 * [1 <= 1*(1*1), 5 <= 5*(1*1), 8 <= 8*(1*1), ]
 *
 * give 6 cars
 * [4 <= 1*(2*2), 5 <= 5*(1*1), 8 <= 8*(1*1), ]
 * [5 <= 5*(1*1), 8 <= 8*(1*1), 9 <= 1*(3*3), ]
 * [8 <= 8*(1*1), 9 <= 1*(3*3), 20 <= 5*(2*2), ]
 * [9 <= 1*(3*3), 20 <= 5*(2*2), 32 <= 8*(2*2), ]
 * [16 <= 1*(4*4), 20 <= 5*(2*2), 32 <= 8*(2*2), ]
 * [20 <= 5*(2*2), 25 <= 1*(5*5), 32 <= 8*(2*2), ]
 *
 * actuall times, not next car times
 * [5 <= 5*(1*1), 16 <= 1*(4*4), 8 <= 8*(1*1), ]
 * Max [5,16,8]
 * result 16
 */
test('case 2', () => {
  const actual = solution([5,1,8], 6);
  expect(actual).toEqual(16);
});


test('case 3', () => {
  const actual = solution([5,4,3,4,10,5,5,4,5,5,3,7,8,7,6,2,10,10,4,1,3,3,4,3,8,5,8,3,6,9,1,8,9,7,2,8,7,1,5,7,10,5,9,5,5,6,8,6], 18);
  expect(actual).toEqual(4);
});


