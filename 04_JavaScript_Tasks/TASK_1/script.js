
const checkSum = (arr2) =>new Promise(function (resolve, reject) {
  var sum = arr2.reduce((a, c) => a + c, 0);
  if (sum > 35) {
    resolve("Promise resolved");
  } else {
    reject("Promise rejected");
  }
});

function firstFunction(arr1) {
  var firstEle = arr1[0];
  arr1.shift();
  console.log("First array :", arr1);
  secondFunction(firstEle, arr1);
}

function secondFunction(ele, arr1) {
  var arr2 = [24, 25, 26, 27, 28, 29];
  arr2.unshift(ele);
  arr2.push(...arr1);
  console.log("Second array :", arr2);
  
  checkSum(arr2)
  .then((result) => console.log(result))
  .catch((error) => console.log(error));
}

var array1 = [10, 11, 12, 13, 14, 15];
firstFunction(array1);


