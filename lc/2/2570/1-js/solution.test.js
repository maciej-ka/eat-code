import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([[1,2],[2,3],[4,5]], [[1,4],[3,2],[4,1]]);
  expect(actual).toEqual([[1,6],[2,3],[3,2],[4,6]]);
});

test('case 2', () => {
  const actual = solution([[2,4],[3,6],[5,5]], [[1,3],[4,3]]);
  expect(actual).toEqual([[1,3],[2,4],[3,6],[4,3],[5,5]]);
});

