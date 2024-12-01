# Day 01: Historian Hysteria [{}](./day01/)
## Part 1: Total Distance Between Lists

The process involves:
1. Sorting both lists.
2. Pairing the smallest number in the left list with the smallest in the right list, the second smallest with the second smallest, and so on.
3. Calculating the **absolute difference** between each pair and summing these distances.
### Examples
Given the lists: \
**Left**:  3 4 2 1 3 3 \
**Right**: 4 3 5 3 9 3

After sorting: \
**Left**:  1 2 3 3 3 4 \
**Right**: 3 3 3 4 5 9

Pairs and distances:
- Pair (1, 3) → Distance: 2
- Pair (2, 3) → Distance: 1
- Pair (3, 3) → Distance: 0
- Pair (3, 4) → Distance: 1
- Pair (3, 5) → Distance: 2
- Pair (4, 9) → Distance: 5

**Total Distance**: `2 + 1 + 0 + 1 + 2 + 5 = 11`

The task is to calculate this total distance for the actual lists provided.

---

## Part 2: Similarity Score Between Lists

### Process:
1. For each number in the left list:
   - Count how many times it appears in the right list.
   - Multiply the number by its count.
2. Add these values to get the **similarity score**.

### Examples:
Given the lists: \
**Left**:  3 4 2 1 3 3 \
**Right**: 4 3 5 3 9 3

Step-by-step calculation:
- `3` appears **3 times** → `3 * 3 = 9`
- `4` appears **1 time** →  `4 * 1 = 4`
- `2` appears **0 times** → `2 * 0 = 0`
- `1` appears **0 times** → `1 * 0 = 0`
- `3` appears **3 times** → `3 * 3 = 9`
- `3` appears **3 times** → `3 * 3 = 9`

**Total Similarity Score**: `9 + 4 + 0 + 0 + 9 + 9 = 31`

The task is to calculate this similarity score for the actual lists provided.