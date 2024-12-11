# Day11: Plutonian Pebbles

## Part 1: Stone Evolution After 25 Blinks

While observing the mysterious stones of Pluto, you notice they evolve according to these rules every time you blink:

### Rules for Stone Transformation:
1. If the stone's number is **0**, it becomes `1`.
2. If the stone's number has an **even number of digits**, it splits into two stones:
   - The left half of the digits becomes one stone.
   - The right half of the digits becomes the other stone.
   - Leading zeroes are dropped.
3. Otherwise, the stone's number is multiplied by **2024** to produce a single new stone.

The stones maintain their order, and all transformations happen simultaneously.

---

### Example Evolution:
Starting arrangement:

`
125 17
`

#### After 1 Blink:
- `125` is multiplied by `2024` → `253000`.
- `17` splits into `1` and `7`.

`
253000 1 7
`

#### After 2 Blinks:
- `253000` splits into `253` and `0`.
- `1` becomes `2024`.
- `7` is multiplied by `2024` → `14168`.

`
253 0 2024 14168
`

#### After 3 Blinks:
- `253` is multiplied by `2024` → `512072`.
- `0` becomes `1`.
- `2024` splits into `20` and `24`.
- `14168` splits into `141` and `68`.

`
512072 1 20 24 28676032
`

#### Continuing the Process:
After blinking **6 times**, the arrangement is:

`
2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
`

- Total stones: **22**.

#### After 25 Blinks:
- The number of stones grows exponentially.
- Total stones: **55312**.

**Goal**: Calculate how many stones exist after 25 blinks. The answer is **55312**.

---

## Part 2: Stone Evolution After 75 Blinks

As you continue observing the stones, their transformations persist according to the same rules. However, over 75 blinks, the number of stones grows astronomically due to the exponential nature of the transformations.

### Example Calculation:
For a smaller case:
- Starting with `125 17`, after 6 blinks: **22 stones**.
- After 25 blinks: **55312 stones**.

#### After 75 Blinks:
Using the same exponential growth patterns, the number of stones becomes extremely large. While exact calculations require simulation or advanced mathematical modeling, the trend suggests growth proportional to the complexity of repeated splits and multiplications.

**Goal**: Calculate the number of stones after 75 blinks. This number would likely exceed trillions, showcasing the scale of growth over time.