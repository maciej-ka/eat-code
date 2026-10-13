import { expect, test } from "vitest";
import solution from "./solution";

// 11
// 22
// 33
// 44
// 55
// 66
// 77
// 88
// 99
test("case 1", () => {
  const actual = solution(1, 100);
  expect(actual).toEqual(9);
});

// 1203
// 1212
// 1221
// 1230
test("case 2", () => {
  const actual = solution(1200, 1230);
  expect(actual).toEqual(4);
});

test("case 3", () => {
  const actual = solution(11, 11);
  expect(actual).toEqual(1);
});

test("case 4", () => {
  const actual = solution(100, 4932);
  expect(actual).toEqual(274);
});
