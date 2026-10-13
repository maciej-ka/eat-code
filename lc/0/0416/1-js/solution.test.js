import { expect, test } from "vitest";
import solution from "./solution";

test("case 1", () => {
  const actual = solution([1,5,11,5]);
  expect(actual).toEqual(true);
});

test("case 2", () => {
  const actual = solution([1,2,3,5]);
  expect(actual).toEqual(false);
});

// 7: 7, 5
// 12: 6, 6
//
// [7,6,6,5]
// total 24
//
// sumA: 7
// ahead: 17
// diff: 17 - 7 = 10
// next possible?: diff - 2*6 < 0
// ... no, next possible
//
// switch, set goal sumA - sumB = 7
// goal: 7
// sumB: 6
// ahead: 11
// diff: 11 + 7 - 6 = 12
// next possible: diff - 2*6 === 0 (yes, and it's end)
test("case 3", () => {
  const actual = solution([7,6,6,5]);
  expect(actual).toEqual(true);
});

// A) add to smaller
// 8: 5 3
// 10: 4 3 3
// 
// B) total ahead (reverse walking sum)
// and keep adding to meet goal
// [5,4,3,3,3]
//
// total ahead (and active)
// [18,13,9,6,3]
//
// goal: 0, keep adding to A
// sumA: 5 (13 rest)
// sumA: 5, 4 (9 rest)
// done
test("case 4", () => {
  const actual = solution([3,3,3,4,5]);
  expect(actual).toEqual(true);
});

test("case 5", () => {
  const actual = solution([1,2,3,4,5,6,7]);
  expect(actual).toEqual(true);
});

test("case 6", () => {
  const actual = solution([14,9,8,4,3,2]);
  expect(actual).toEqual(true);
});

test("case 7", () => {
  const actual = solution([20,10,9,8,8,3])
  expect(actual).toEqual(true)
})

