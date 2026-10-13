import { expect, test } from "vitest";
import solution from "./solution";

test("case 1", () => {
  const actual = solution(1);
  expect(actual).toEqual(5);
});

// odd: 0,2,4,6,8
// prime: 2,3,5,7
// 5 * 4 * 5 * 4
test("case 2", () => {
  const actual = solution(4);
  expect(actual).toEqual(400);
});

test("case 3", () => {
  const actual = solution(50);
  expect(actual).toEqual(564908303);
});

test("case 4", () => {
  const actual = solution(1924);
  expect(actual).toEqual(805821919);
});

test("case 5", () => {
  const actual = solution(84);
  expect(actual).toEqual(736805306);
});

test("case 6", () => {
  const actual = solution(85);
  expect(actual).toEqual(684026509);
});

test.only("case 7", () => {
  const actual = solution(806166225460393);
  expect(actual).toEqual(643535977);
});
