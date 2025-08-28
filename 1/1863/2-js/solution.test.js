import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([1,3]);
  expect(actual).toEqual(6);
});

// [5,1,6]
// ['101', '001', '110']
//
// 000 []: 0 (0)
// 101 [5]: 101 (5)
// 001 [1]: 1 (1)
// 110 [6]: 110 (6)
// 100 [5,1]: 101 ^ 1 = 100 (4)
// 011 [5,6]: 101 ^ 110 = 011 (3)
// 111 [1,6]: 1 ^ 110 = 111 (7)
// 010 [5,1,6]: 101 ^ 1 ^ 110 = 010 (2)
//
// sum of all XOR totals:
// 0 + 5 + 1 + 6 + 4 + 3 + 7 + 2
// 28

test('case 2', () => {
  const actual = solution([5,1,6]);
  expect(actual).toEqual(28);
});

test('case 3', () => {
  const actual = solution([3,4,5,6,7,8]);
  expect(actual).toEqual(480);
});

test('case 4', () => {
  const actual = solution([1,1,1]);
  expect(actual).toEqual(4);
});

// [5,1,2]
// ['101', '001', '010']
// 
// 000 []: 0 (0)
// 101 [5]: 101 (5)
// 001 [1]: 001 (1)
// 010 [2]: 010 (2)
// 100 [5,1]: 101 ^ 001 = 100 (4)
// 111 [5,2]: 101 ^ 010 = 111 (7)
// 011 [1,2]: 001 ^ 010 = 011 (3)
// 110 [5,1,2]: 101 ^ 001 ^ 010 = 110 (6)
//
// sum of all XOR totals:
// 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7
// 28

test('case 2', () => {
  const actual = solution([5,1,2]);
  expect(actual).toEqual(28);
});

