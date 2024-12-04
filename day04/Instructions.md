# Day04: Ceres Search

## Part 1: Finding "XMAS"
The Elf on the Ceres monitoring station needs help finding the word **XMAS** in a word search puzzle. The word can appear:
- Horizontally, vertically, or diagonally.
- Forwards or backwards.
- Overlapping other words.

For example, **XMAS** can appear like this:
```
..X…
.SAMX.
.A..A.
XMAS.S
.X….
```

In a larger grid, you must count **all** occurrences of the word. For example, in the following puzzle:

```
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
```

The word **XMAS** appears **18 times**.

**Objective:** Count how many times **XMAS** appears in the given word search puzzle.

---

## Part 2: Finding "X-MAS"
Upon reviewing the puzzle, you realize the real challenge is finding **X-MAS**, which consists of two **MAS** patterns arranged in an X shape. Here's an example of an X-MAS pattern:
```
.M.S……
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M….
……….
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
……….
```

The **X-MAS** pattern appears **9 times**.

**Objective:** Count how many times the **X-MAS** pattern appears in the given word search puzzle.
