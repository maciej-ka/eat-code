import { expect, test } from "vitest";
import solution from "./solution";

// 6000, 4, 124
// free: 1
// first needs deduce: yes
// first options: 6
//
// each digit can be at most 4
// (0...4)124 = 5 possibilities
test("case 1", () => {
  const actual = solution(1, 6000, 4, "124");
  expect(actual).toEqual(5);
});

// "215", 6, "10"
// first options 3
// "15", 6, "10"
// free === 0
//
//
// (0...2)"10" = 3
// - 1
// 2
test("case 2", () => {
  const actual = solution(15, 215, 6, "10");
  expect(actual).toEqual(2);
});

test("case 3", () => {
  const actual = solution(1000, 2000, 4, "3000");
  expect(actual).toEqual(0);
});

// 971, 9 , 17
// free: 1
// optionsFirst: 10
test("case 4", () => {
  const actual = solution(1, 971, 9, "17");
  expect(actual).toEqual(10);
});

// "1159", 5, "20"
// free: 2
// optionsFirst: 2
// optionsRest aaaah
//
// (0)(0...5)
// (1)(0...1) (not 0...5)
test.only("case 5", () => {
  const actual = solution(20, 1159, 5, "20");
  expect(actual).toEqual(8);
});

