const myPromise = (sum) => new Promise((resolve, reject) => {
  console.log(sum);
  if (sum > 35) {
    resolve("promise resolved");
  } else {
    reject("promis rejected");
  }
});

function fun1() {
  let arr = [1, 2, 3];
  let mySymbol = Symbol();
  let symbolArray = {
    [mySymbol]: arr,
  };
  fun2(symbolArray, mySymbol);
}

function fun2(symbolArray, mySymbol) {

  let element = symbolArray[mySymbol].shift();

  console.log("removed element",element);
  
  let arr2 = [24, 25, 26];
  arr2.unshift(element);
  arr2.push(...symbolArray[mySymbol]);
  
  console.log(arr2);
  
  let sum = arr2.reduce((sum, c) => sum + c, 0);
  console.log(sum);
  
  myPromise(sum)
    .then((message) => console.log(message))
    .catch((error) => console.log(error));
}
fun1();


