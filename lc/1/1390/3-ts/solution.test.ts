import { expect, test } from "vitest";
import solution from "./solution.js";

test("test 1", () => {
  const actual = solution([21, 4, 7]);
  const expected = 32;
  expect(actual).toEqual(expected);
})

test("test 2", () => {
  const actual = solution([21, 21]);
  const expected = 64;
  expect(actual).toEqual(expected);
})

test("test 3", () => {
  const actual = solution([1, 2, 3, 4, 5]);
  const expected = 0;
  expect(actual).toEqual(expected);
})

