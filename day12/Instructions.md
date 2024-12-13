# Day12: Garden Groups

## Part 1:
### Key Definitions:
- **Area**: The number of plots in a region.
- **Perimeter**: The number of sides of plots in the region that do not touch other plots of the same region.
- **Price**: Calculated as:
  \[
  Price = Area * Perimeter
  \]

### Example:
For the map:
```
AAAA
BBCD
BBCC
EEEC
```
- **Region A**: Area = 4, Perimeter = 10 → Price = \(4 * 10 = 40\)
- **Region B**: Area = 4, Perimeter = 8 → Price = \(4 * 8 = 32\)
- **Region C**: Area = 4, Perimeter = 10 → Price = \(4 * 10 = 40\)
- **Region D**: Area = 1, Perimeter = 4 → Price = \(1 * 4 = 4\)
- **Region E**: Area = 3, Perimeter = 8 → Price = \(3 * 8 = 24\)

**Total Price**: \(40 + 32 + 40 + 4 + 24 = 140\)

For the map:
```
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
```
- **Region O**: Area = 21, Perimeter = 36 → Price = \(21 * 36 = 756\)
- **Each X Region**: Area = 1, Perimeter = 4 → Price = \(1 * 4 = 4\)

**Total Price**: \(756 + 4 + 4 + 4 + 4 = 772\)

### Larger Example:
For the map:
```
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
```
- Total Price: **1930**

---

## Part 2: Calculating Fence Price Using Area and Number of Sides

With a bulk discount, the price is now calculated based on the **number of sides** in each region instead of the perimeter. Each **straight section of fence** (regardless of its length) counts as one side.

### Key Definitions:
- **Sides**: The number of straight sections of fence enclosing the region.
- **Price**: Calculated as:
  \[
  Price = Area * Number of Sides
  \]

### Example:
For the map:
```
AAAA
BBCD
BBCC
EEEC
```
- **Region A**: Area = 4, Sides = 4 → Price = \(4 * 4 = 16\)
- **Region B**: Area = 4, Sides = 4 → Price = \(4 * 4 = 16\)
- **Region C**: Area = 4, Sides = 8 → Price = \(4 * 8 = 32\)
- **Region D**: Area = 1, Sides = 4 → Price = \(1 * 4 = 4\)
- **Region E**: Area = 3, Sides = 4 → Price = \(3 * 4 = 12\)

**Total Price**: \(16 + 16 + 32 + 4 + 12 = 80\)

For the map:
```
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
```
- **Region E**: Area = 17, Sides = 12 → Price = \(17 * 12 = 204\)
- **Each X Region**: Price = \(1 * 4 = 4\)

**Total Price**: \(204 + 4 + 4 = 236\)

### Larger Example:
For the map:
```
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
```
- Total Price: **1206**
