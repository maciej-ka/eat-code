import { expect, test } from 'vitest'
import solution from "./solution";

test.only('case 1', () => {
  const actual = solution(7, [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]);
  expect(actual).toEqual();
});

test('case 2', () => {
  const actual = solution(2, [[1,0,10]]);
  expect(actual).toEqual();
});

