import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([1,-3,2,3,-4]);
  expect(actual).toEqual(5);
});

test('case 2', () => {
  const actual = solution([2,-5,1,-4,3,-2]);
  expect(actual).toEqual(8);
});
