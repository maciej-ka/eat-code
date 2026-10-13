import { expect, test } from "vitest";
import solution from "./solution.js";

test("test 1", () => {
  const actual = solution([1, 2, 3]);
  const expected = 3;
  expect(actual).toEqual(expected);
})

