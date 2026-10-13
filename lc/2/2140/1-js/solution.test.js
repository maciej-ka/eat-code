import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([[3,2],[4,3],[4,4],[2,5]]);
  expect(actual).toEqual(5);
});

test('case 2', () => {
  const actual = solution([[1,1],[2,2],[3,3],[4,4],[5,5]]);
  expect(actual).toEqual(7);
});
