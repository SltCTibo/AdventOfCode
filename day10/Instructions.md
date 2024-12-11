# Day10: Hoof It

## Part 1: Scoring Trailheads

While exploring a floating island's Lava Production Facility, you assist a reindeer trying to reconstruct a hiking trail map. The goal is to identify **trailheads** and calculate their **scores** based on the hiking trails they connect to.

### Topographic Map Basics:
- Each position has a height from `0` (lowest) to `9` (highest).
- **Hiking Trails**:
  - Start at a height of `0`.
  - End at a height of `9`.
  - Increase by exactly `1` at each step.
  - Only move up, down, left, or right (no diagonal steps).
- **Trailheads**: Positions with a height of `0` where trails begin.
- **Trailhead Score**: The number of `9-height` positions reachable via a hiking trail.

### Example Maps:
1. Simple Map:

```
0123
1234
8765
9876
```

- Single trailhead at the top-left (`0`).
- Score: 1 (connects to the `9` in the bottom-left).

2. Complex Map:

```
..90..9
…1.98
…2..7
6543456
765.987
876….
987….
```

- Trailhead in the top-left:
  - Score: 4 (connects to all but one `9`).

3. Larger Map:

```
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
```

- 9 trailheads with scores: 5, 6, 5, 3, 1, 3, 5, 3, and 5.
- Total score: **36**.

**Goal**: Calculate the **sum of all trailhead scores** for your map.

---

## Part 2: Rating Trailheads

After creating the initial hiking trail map, you discover a new metric for trailheads: **rating**. This measures the number of **distinct hiking trails** beginning at a trailhead.

### New Rules:
- A **distinct trail** is a unique path from the trailhead to a `9`.
- Trail ratings count all possible valid paths.

### Example Maps:
1. Simple Map:

```
…..0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9….
```

- Trailhead at the top-center (`0`).
- Rating: 3 (three distinct trails to `9`).

2. Complex Map:

```
..90..9
…1.98
…2..7
6543456
765.987
876….
987….
```

- Single trailhead at the top-left:
  - Rating: 13 (distinct trails to `9` on the edges).

3. Larger Map:

```
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
```

- Trailhead ratings: 20, 24, 10, 4, 1, 4, 5, 8, and 5.
- Total rating: **81**.

**Goal**: Calculate the **sum of all trailhead ratings** for your map.