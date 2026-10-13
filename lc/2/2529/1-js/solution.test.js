import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([-2,-1,-1,1,2,3]);
  expect(actual).toEqual(3);
});

test('case 2', () => {
  const actual = solution([-3,-2,-1,0,0,1,2]);
  expect(actual).toEqual(3);
});

test('case 3', () => {
  const actual = solution([5,20,66,1314]);
  expect(actual).toEqual(4);
});

test('case 4', () => {
  const actual = solution([0]);
  expect(actual).toEqual(0);
});

test('case 5', () => {
  const actual = solution([1]);
  expect(actual).toEqual(1);
});

test('case 6', () => {
  const actual = solution([-1]);
  expect(actual).toEqual(1);
});

test('case 7', () => {
  const actual = solution([0,1]);
  expect(actual).toEqual(1);
});

test('case 8', () => {
  const actual = solution([1, 2]);
  expect(actual).toEqual(2);
});

test('case 9', () => {
  const actual = solution([0, 0]);
  expect(actual).toEqual(0);
});

test('case 10', () => {
  const actual = solution([-100, -99, -88]);
  expect(actual).toEqual(3);
});
