/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */
export function cookingStatus(timer = 'You forgot to set the timer.'){
  return (timer === 0) ? 'Lasagna is done.' : (timer >0) ? 'Not done, please wait.' : timer;
}

export function preparationTime(layers, avgTime = 2){
  return layers.length * avgTime;
}

export function quantities (layers) {
  let ans = {
    noodles:0,
    sauce:0
  }
  for(let layer of layers){
    switch(layer){
      case 'noodles':
        ans.noodles += 50;
        break;
      case 'sauce':
        ans.sauce += 0.2;
        break;
    }
  }
  return ans;
}

export function addSecretIngredient (friendsList, myList) {
  let secretIngredient = friendsList[friendsList.length - 1];
  myList.push(secretIngredient);
}

export function scaleRecipe(recipe, noOfPortion) {
  let mul = noOfPortion/2;
  let ans = {};
  for(let key in recipe){
    ans[key] = recipe[key]*mul;
  }
  return ans;
}