# Day08: Resonant Collinearity

## Part 1: Calculating Antinodes from Signal Interference

### Rules for Antinode Creation:
1. Antinodes are created by antennas of the **same frequency**.
2. An antinode occurs at any point perfectly in line with two antennas of the same frequency, where **one antenna is twice as far** from the point as the other.
3. Different frequencies (e.g., `A` vs `a`) do not interact to create antinodes.

### Example Map:

```
…………
……..0…
…..0……
…….0….
….0…….
……A…..
…………
…………
……..A…
………A..
…………
…………
```

Antinodes for antennas with frequency `a`:

```
……….
…#……
……….
….a…..
……….
…..a….
……….
……#…
……….
……….
```

Adding more antennas with the same frequency creates additional antinodes:

```
……….
…#……
#………
….a…..
……..a.
…..a….
..#…….
……#…
……….
……….
```

### Total Antinode Locations:
- **14 unique locations** within the map bounds contain an antinode.

---

## Part 2: Incorporating Resonant Harmonics

One of The Historians realizes the model needs an update to account for **resonant harmonics**:
- Antinodes occur at any position in line with **at least two antennas** of the same frequency, regardless of distance.
- This includes the positions of the antennas themselves, as long as there are at least two antennas of the same frequency.

### Updated Example Map:

```
T….#….
…T……
.T….#…
………#
..#…….
……….
…#……
……….
….#…..
……….
```

Antinodes now include antenna positions, resulting in **9 total antinodes** in this example.

### Original Map Revisited:
With resonant harmonics applied, the original map now looks like this:

```
##….#….#
.#.#….0…
..#.#0….#.
..##…0….
….0….#..
.#…#A….#
…#..#…..
#….#.#….
..#…..A…
….#….A..
.#……..#.
…#……##
```

### Total Antinode Locations:
- **34 unique locations** within the map bounds now contain an antinode.

This updated model significantly increases the impact of the antennas, highlighting the role of resonant harmonics in expanding the signal’s reach.