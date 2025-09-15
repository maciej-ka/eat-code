import { expect, test } from "vitest";
import canBeTypedWords from "./solution";

test("test 1", () => {
  const actual = canBeTypedWords("hello world", "ad");
  const expected = 1;
  expect(actual).toEqual(expected);
});

test("test 2", () => {
  const actual = canBeTypedWords("leet code", "lt");
  const expected = 1;
  expect(actual).toEqual(expected);
});

test("test 3", () => {
  const actual = canBeTypedWords("leet code", "e");
  const expected = 0;
  expect(actual).toEqual(expected);
});

