// https://leetcode.com/problems/design-a-food-rating-system/submissions/1773393539
import _ from 'lodash';
import {} from 'datastructures-js';

/**
 * Your FoodRatings object will be instantiated and called as such:
 * var obj = new FoodRatings(foods, cuisines, ratings)
 * obj.changeRating(food,newRating)
 * var param_2 = obj.highestRated(cuisine)
 */
class FoodRatings {

  /**
   * @param {string[]} foods
   * @param {string[]} cuisines
   * @param {number[]} ratings
   */
  constructor(foods, cuisines, ratings) {
    // food => cousine
    this.cousineLookup = {}

    // cousine => Heap
    this.heaps = {}

    for (let i = 0; i < foods.length; i ++) {
      this.cousineLookup[foods[i]] = cuisines[i];
      if (!this.heaps[cuisines[i]]) this.heaps[cuisines[i]] = new FoodHeap();
      this.heaps[cuisines[i]].push(foods[i], ratings[i])
    }
  }

  /**
   * @param {string} food
   * @param {number} newRating
   * @return {void}
   */
  changeRating(food, newRating) {
    this.heaps[this.cousineLookup[food]].update(food, newRating);
  }

  /**
   * @param {string} cuisine
   * @return {string}
   */
  highestRated(cuisine) {
    return this.heaps[cuisine].peek();
  }
}


class FoodHeap {

  constructor() {
    this.foods = [];
    this.ratings = [];
    // food => index
    this.indexLookup = {};
  }

  peek() {
    return this.foods[0];
  }

  swap(i, k) {
    let temp;

    this.indexLookup[this.foods[i]] = k;
    this.indexLookup[this.foods[k]] = i;

    temp = this.foods[i];
    this.foods[i] = this.foods[k];
    this.foods[k] = temp;

    temp = this.ratings[i];
    this.ratings[i] = this.ratings[k];
    this.ratings[k] = temp;
  }

  isGreater(i, k) {
    return this.ratings[i] > this.ratings[k]
      || this.ratings[i] === this.ratings[k] && this.foods[i] < this.foods[k]
  }

  bubbleUp(i) {
    if (i === 0) return;
    const p = i - 1 >> 1
    if (this.isGreater(i, p)) {
      this.swap(i, p);
      this.bubbleUp(p);
    }
  }

  bubbleDown(i) {
    let c = (i << 1) + 1;
    if (c >= this.foods.length) return;

    const c2 = c + 1;
    if (c2 < this.foods.length && this.isGreater(c2, c)) {
      c = c2;
    }

    if (this.isGreater(c, i)) {
      this.swap(c, i);
      this.bubbleDown(c);
    }
  }

  push(food, rating) {
    this.foods.push(food);
    this.ratings.push(rating);
    this.indexLookup[food] = this.foods.length - 1;
    this.bubbleUp(this.foods.length - 1);
  }

  update(food, rating) {
    const i = this.indexLookup[food];
    const oldRating = this.ratings[i];
    this.ratings[i] = rating;
    rating > oldRating
      ? this.bubbleUp(i)
      : this.bubbleDown(i);
  }

  debug() {
    console.log('--------------');
    console.log(this.indexLookup);
    console.log(this.foods);
    console.log(this.ratings);
  }
}


export default FoodRatings;
