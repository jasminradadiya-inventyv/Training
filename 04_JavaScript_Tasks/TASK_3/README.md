# Array Manipulation Using Symbol and Promise

## Description
This program demonstrates how JavaScript Symbols can be used as object keys, how arrays are modified, and how a Promise is used to validate the sum of array elements.

---

## Approach
- An array is stored inside an object using a Symbol as the key.
- One element is removed from the array and reused.
- A new array is created by combining values.
- The sum is checked using a Promise.

---

## How It Works
- `fun1` creates a Symbol and stores an array using it.
- `fun2` removes the first element and builds a new array.
- The total sum is calculated and passed to a Promise.
- The Promise resolves or rejects based on the sum value.

---

## Concepts Used
- Symbols
- Arrays and array methods
- Spread operator
- Promises

---

## Output
The program logs the removed element, the final array, the total sum, and the Promise result.
