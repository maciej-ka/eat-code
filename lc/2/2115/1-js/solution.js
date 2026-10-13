/**
 * @param {string[]} recipes
 * @param {string[][]} ingredients
 * @param {string[]} supplies
 * @return {string[]}
 */

var findAllRecipes = function(recipes, ingredients, supplies) {
  // result in desired format
  const result = []

  // fast way to get ingredient list
  // false value means recipe was already attempted
  const recipeMap = {}
  for (let i in recipes) { recipeMap[recipes[i]] = ingredients[i] }

  // fast lookup of supplies
  const supplySet = new Set(supplies)

  // check using recursion
  function check(item) {
    // it's either supply that we have or recipe we can make
    if (supplySet.has(item)) { return true }

    // it's a supply we don't have
    if (!(item in recipeMap)) { return false }

    // it's a recipe we cannot make
    const recipe = recipeMap[item]
    if (!recipe) { return false }

    // it's a recipe to process
    recipeMap[item] = false
    for (let igredient of recipe) {
      if (!check(igredient)) { return false }
    }

    // recipe is doable
    result.push(item)
    supplySet.add(item)
    return true
  }

  for (let recipe of recipes) { check(recipe) }
  return result
};

export default findAllRecipes
