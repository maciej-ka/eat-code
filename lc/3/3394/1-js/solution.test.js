import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(5,[[1,0,5,2],[0,2,2,4],[3,2,5,3],[0,4,4,5]]);
  expect(actual).toEqual(true);
});

test('case 2', () => {
  const actual = solution(4,[[0,0,1,1],[2,0,3,4],[0,2,2,3],[3,0,4,3]]);
  expect(actual).toEqual(true);
});

test('case 3', () => {
  const actual = solution(4,[[0,2,2,4],[1,0,3,2],[2,2,3,4],[3,0,4,2],[3,2,4,4]]);
  expect(actual).toEqual(false);
});
