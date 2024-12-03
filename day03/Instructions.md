# Day 03: Mull It Over

## Part 1: Scanning Corrupted Memory
The North Pole Toboggan Rental Shop's computer is malfunctioning again! Its memory is corrupted, jumbled with invalid characters, and only some `mul(X,Y)` instructions are valid. Each `mul` instruction multiplies two numbers, `X` and `Y`, and adds the result to the total.

For example, in the corrupted memory:

```
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
```

Only the valid `mul` instructions are processed:
- `mul(2,4)` = 8
- `mul(3,7)` = 21
- `mul(32,64)` = 2048
- `mul(8,5)` = 40

The total result is `161`.

**Objective:** Scan the corrupted memory, extract valid `mul` instructions, and calculate the sum of their results.

## Part 2: Adding Conditional Instructions
While scanning corrupted memory, additional conditional instructions appear:
- `do()`: Enables processing of future `mul` instructions.
- `don't()`: Disables processing of future `mul` instructions.

At the start of the program, `mul` instructions are enabled. The most recent `do()` or `don't()` instruction determines whether subsequent `mul` instructions are processed.

For example:
```
xmul(2,4)&mul[3,7]!^don’t()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
```

Here, only the following are processed:
- `mul(2,4)` = 8 (enabled at start)
- `mul(8,5)` = 40 (re-enabled by `do()`)

The total result is `48`.

**Objective:** Handle the `do()` and `don't()` instructions, and calculate the sum of the results of only the enabled multiplications.
