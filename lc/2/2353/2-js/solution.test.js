import { expect, test } from "vitest";
import FoodRatings from "./solution";

test("test 1", () => {
  const obj = new FoodRatings(
    ["kimchi","miso","sushi","moussaka","ramen","bulgogi"],
    ["korean","japanese","japanese","greek","japanese","korean"],
    [9,12,8,15,14,7]
  );
  expect(obj.highestRated("korean")).toEqual("kimchi");
  expect(obj.highestRated("japanese")).toEqual("ramen");
  obj.changeRating("sushi", 16);
  expect(obj.highestRated("japanese")).toEqual("sushi");
  obj.changeRating("ramen", 16);
  expect(obj.highestRated("japanese")).toEqual("ramen");
})

test("test 2", () => {
  const obj = new FoodRatings(
    [ "xxdcg", "wfqdeytt", "jqmfm", "ukqbjikyx", "aymciznrnw", "qhjjrvr", "wzcinxg", "ikxj", ],
    [ "lruhtqy", "lruhtqy", "lruhtqy", "lruhtqy", "lruhtqy", "lruhtqy", "lruhtqy", "lruhtqy", ],
    [8, 6, 1, 17, 20, 2, 17, 14]
  );
  expect(obj.highestRated("lruhtqy")).toEqual("aymciznrnw");
  obj.changeRating("wfqdeytt", 17)
  obj.changeRating("aymciznrnw", 9)
  expect(obj.highestRated("lruhtqy")).toEqual("ukqbjikyx");
  obj.changeRating("ukqbjikyx", 10)
  expect(obj.highestRated("lruhtqy")).toEqual("wfqdeytt");
  obj.changeRating("xxdcg", 11)
  obj.changeRating("ikxj", 15)
  obj.changeRating("aymciznrnw", 10)
  obj.changeRating("wfqdeytt", 10)
  obj.changeRating("xxdcg", 16)
  obj.changeRating("ikxj", 2)
  obj.changeRating("aymciznrnw", 16)
  expect(obj.highestRated("lruhtqy")).toEqual("wzcinxg");
  obj.changeRating("wzcinxg", 12)
  expect(obj.highestRated("lruhtqy")).toEqual("aymciznrnw");
})

