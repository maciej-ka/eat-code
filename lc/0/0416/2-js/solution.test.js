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

test("case 3", () => {
  const actual = solution([7,6,6,5]);
  expect(actual).toEqual(true);
});

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

// 29: 10, 8, 8, 3
// 29: 20, 9
//
// is there permutation in which equilibrum will work?
// [20, 10, 8, 8, 9, 3]
test("case 7", () => {
  const actual = solution([20,10,9,8,8,3])
  expect(actual).toEqual(true)
})

