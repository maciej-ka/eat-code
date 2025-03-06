import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([[1,3],[2,2]]);
  expect(actual).toEqual([2,4]);
});

test('case 2', () => {
  const actual = solution([[9,1,7],[8,9,2],[3,4,6]]);
  expect(actual).toEqual([9,5]);
});

