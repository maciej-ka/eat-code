import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  //                       0123456789
  const actual = solution("WBBWWBBWBW", 7);
  expect(actual).toEqual(3);
});

test('case 2', () => {
  const actual = solution("WBWBBBW", 2);
  expect(actual).toEqual(0);
});

