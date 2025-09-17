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

