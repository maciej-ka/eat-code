import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([9,12,5,10,14,3,10], 10);
  expect(actual).toEqual([9,5,3,10,10,12,14]);
});

test('case 2', () => {
  const actual = solution([-3,4,3,2], 2);
  expect(actual).toEqual([-3,2,4,3]);
});

