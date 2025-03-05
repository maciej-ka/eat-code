import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(1);
  expect(actual).toEqual(1);
});

test('case 2', () => {
  const actual = solution(2);
  expect(actual).toEqual(5);
});

test('case 3', () => {
  const actual = solution(3);
  expect(actual).toEqual(13);
});

