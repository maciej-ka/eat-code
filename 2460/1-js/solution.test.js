import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([1,2,2,1,1,0]);
  expect(actual).toEqual([1,4,2,0,0,0]);
});

test('case 2', () => {
  const actual = solution([0,1]);
  expect(actual).toEqual([1,0]);
});

