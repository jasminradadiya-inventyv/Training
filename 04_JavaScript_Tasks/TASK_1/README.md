# Array Handling and Used it with JavaScript Promise

## Description
This program manipulates arrays using multiple functions and checks the sum of a final array using a JavaScript Promise.

---

## Approach
- The first element of the original array is removed and stored.
- A new array is created.
- It combining that removed element from first array at start position of second array, full second array and then remaining elements from first array.
- A Promise checks whether the sum of the final array is greater than 35.

---

## How It Works
- `firstFunction` removes the first element and passes data to the next function.
- `secondFunction` creates a new array and calls the Promise.
- `checkSum` calculates the total sum and resolves or rejects based on the condition.

---

## Concepts Used
- Functions
- Array methods - shift, unshift, push
- Spread operator
- Promises

---

## Output
The program prints both arrays and displays whether the Promise is resolved or rejected.
