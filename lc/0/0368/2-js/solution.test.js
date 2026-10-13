import { expect, test } from "vitest";
import solution from "./solution";

test("case 1", () => {
  const actual = solution([1, 2, 3]);
  expect(actual).toEqual([1, 3]);
});

test("case 2", () => {
  const actual = solution([1, 2, 4, 8]);
  expect(actual).toEqual([1, 2, 4, 8]);
});

test("case 3", () => {
  const actual = solution([3, 9, 12, 18, 48, 96]);
  expect(actual).toEqual([3, 12, 48, 96]);
});

test("case 4", () => {
  const actual = solution([2, 4, 6, 12, 24]);
  expect(actual).toEqual([2, 6, 12, 24]);
});

test("case 5", () => {
  const actual = solution([3, 6, 8, 24, 72]);
  expect(actual).toEqual([3, 6, 24, 72]);
});

test("case 6", () => {
  const actual = solution([3, 5, 10, 30, 60, 100, 315]);
  expect(actual).toEqual([5, 10, 30, 60]);
});

test("case 7", () => {
  const actual = solution([3, 10, 15, 30, 40, 60]);
  expect(actual).toEqual([3, 15, 30, 60]);
});

test("case 8", () => {
  const actual = solution([2, 10, 15, 30, 60, 165]);
  expect(actual).toEqual([2, 10, 30, 60]);
});

