import { expect, test } from "vitest";
import TaskManager from "./solution";

test("test 1", () => {
  const obj = new TaskManager(
    [
      [1, 101, 10],
      [2, 102, 20],
      [3, 103, 15],
    ]
  );
  obj.add(4, 104, 5);
  obj.edit(102, 8);
  expect(obj.execTop()).toEqual(3);
  obj.rmv(101);
  obj.add(5, 105, 15)
  expect(obj.execTop()).toEqual(5);
})

test("test 2", () => {
  const obj = new TaskManager(
    [
      [10, 4, 10],
      [10, 0, 6],
      [5, 23, 50],
      [3, 29, 50],
      [2, 15, 9],
    ]
  );
  expect(obj.execTop()).toEqual(3);
})

test("test 3", () => {
  const obj = new TaskManager(
    [
      [2, 15, 32],
      [6, 4, 19],
      [9, 11, 45],
    ]
  );
  obj.rmv(4);
  obj.edit(15, 26)
  expect(obj.execTop()).toEqual(9);
})

test("test 4", () => {
  const obj = new TaskManager(
    [
      [1, 101, 8],
      [2, 102, 20],
      [3, 103, 5],
    ]
  );
  obj.add(4, 104, 5);
  obj.edit(102, 9);
  expect(obj.execTop()).toEqual(2);
  obj.rmv(101);
  obj.add(50, 101, 8);
  expect(obj.execTop()).toEqual(50);
})

