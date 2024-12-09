# Day07: Bridge Repair

## Part 1: Determining Valid Equations with + adn *

### Algorithm used: Decision Tree

The Historians encounter engineers trying to repair a frequently breaking rope bridge. To finish their work, the engineers need help solving calibration equations. These equations, provided as input, are missing operators, and the task is to determine if the numbers in each equation can be combined using addition (`+`) and multiplication (`*`) to match a target value.

### Rules:
- The target value is given before a colon (`:`) on each line.
- The remaining numbers must be combined using `+` and `*`.
- Operators are evaluated **left-to-right**, ignoring normal precedence rules.
- Numbers **cannot** be rearranged.

### Example:

Input:

```
190: 10 19
3267: 81 40 27
292: 11 6 16 20
```

Explanation:
1. `190: 10 19`:
   - Possible combinations: `10 + 19 = 29`, `10 * 19 = 190`.
   - Result: Valid, as `10 * 19 = 190`.

2. `3267: 81 40 27`:
   - Possible combinations: `(81 + 40) * 27 = 3267` and `81 * (40 + 27) = 3267`.
   - Result: Valid, as it matches the target.

3. `292: 11 6 16 20`:
   - Only one valid combination: `11 + (6 * 16) + 20 = 292`.
   - Result: Valid, as it matches the target.

The total calibration result is the sum of all valid target values. For this example:

```
190 + 3267 + 292 = 3749
```
**Goal**: Find the sum of target values from all valid equations.

## Part 2: Adding Concatenation (||)

The engineers discover a hidden operator: **concatenation** (`||`), which joins two numbers by appending their digits. For example:
- `12 || 345 = 12345`.

Now, you must also account for equations that can be solved using any combination of `+`, `*`, and `||`.

### Rules:
1. Use `+`, `*`, and `||` operators.
2. Operators are evaluated **left-to-right**.
3. Numbers **cannot** be rearranged.

### Example:
Input:
```
156: 15 6
7290: 6 8 6 15
192: 17 8 14
```
Valid equations:
- `156: 15 || 6 = 156`
- `7290: 6 * 8 || 6 * 15 = 7290`
- `192: 17 || 8 + 14 = 192`

**New Total Calibration Result**:
Adding the results from **Part 1** (`190 + 3267 + 292 = 3749`) and these new equations (`156 + 7290 + 192 = 7638`):
```
3749 + 7638 = 11387
```
---

## Summary

### New Rules:
1. Combine numbers using `+`, `*`, and `||`.
2. Evaluate all operators **left-to-right**.
3. Keep the numbers in their original order.

### New Goal:
Determine which equations can be made true with the three operators and calculate the total calibration result.

For the example above, the total calibration result is:
