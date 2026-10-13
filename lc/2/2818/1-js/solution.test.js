import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([8,3,9,3,8], 2);
  expect(actual).toEqual(81);
});

test('case 2', () => {
  const actual = solution([19,12,14,6,10,18], 3);
  expect(actual).toEqual(4788);
});

test('case 3', () => {
  const actual = solution([3289,2832,14858,22011], 6);
  expect(actual).toEqual(256720975);
});

test('case 4', () => {
  const actual = solution([1,1,2,18,1,9,3,1], 23);
  expect(actual).toEqual(275402880);
});

test('case 5', () => {
  const actual = solution([1,1,2,18,1,9,3,1], 24);
  expect(actual).toEqual(478625906);
});

test('case 6', () => {
  const actual = solution([1,1,2,18,1,9,3,1], 32);
  expect(actual).toEqual(346264255);
});

test('case 7', () => {
  const actual = solution([1,1,2,18,1,9,3,1], 18);
  expect(actual).toEqual(682976327);
});

test('case 8', () => {
  const actual = solution([2,1,14,5,18,1,8,5], 34);
  expect(actual).toEqual(799392504);
});

test('case 9', () => {
  const actual = solution([2,1,14,5,18,1,8,5], 19);
  expect(actual).toEqual(693715665);
});
