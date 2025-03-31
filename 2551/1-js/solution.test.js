import { expect, test } from 'vitest'
import solution from "./solution";

// min: 6
// k1 2: 1
// k2 4: 3, 1
//
// max: 10
// k1 4: 1, 3
// k2 6: 5, 1
//
// result: 4
test('case 1', () => {
  const actual = solution([1,3,5,1], 2);
  expect(actual).toEqual();
});

// max and min same
// k1 2: 1
// k2 6: 3
//
// result: 0
test('case 2', () => {
  const actual = solution([1,3], 2);
  expect(actual).toEqual();
});
