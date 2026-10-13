import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([0,1,0,1,0], 3);
  expect(actual).toEqual(3);
});

test('case 2', () => {
  const actual = solution([0,1,0,0,1,0,1], 6);
  expect(actual).toEqual(2);
});

test('case 3', () => {
  const actual = solution([1,1,0,1], 4);
  expect(actual).toEqual(0);
});

test('case 4', () => {
  const actual = solution([1,1,0,1,0], 3);
  expect(actual).toEqual(3);
});

test('case 5', () => {
  const actual = solution([0,1,0,1,0,1], 3);
  expect(actual).toEqual(6);
});

