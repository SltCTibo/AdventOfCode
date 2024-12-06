# Day06: Guard Patrol Challenges

## Part 1: Predicting the Guard's Patrol Path

The Historians are navigating a North Pole lab in the year 1518. To avoid a patrolling guard, you must predict the guard's patrol route based on their strict movement protocol:

```
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
```

1. If there is an obstruction (`#`) directly in front of the guard, they turn 90 degrees to the right.
2. Otherwise, they move one step forward in the direction they are currently facing.

Given a map with the guard's starting position (`^`, `v`, `<`, `>`), you need to track all positions visited by the guard until they leave the map. For example:
- The guard moves forward until encountering an obstacle, turns right, and continues the process.
- The positions visited are marked with `X`, including the starting position.

**Goal**: Calculate the total number of distinct positions the guard visits before leaving the map.

---

## Part 2: Trapping the Guard in a Loop

To safely explore the lab, The Historians propose adding a single new obstruction (`O`) to trap the guard in a loop. This ensures the guard patrols a small, predictable area without ever leaving the map.

- The new obstruction must be strategically placed so that the guard gets stuck.
- The guard’s starting position is not a valid location for the new obstruction.

For example, in the given map:
- The guard can be trapped by adding an obstruction near their starting position, in the bottom-right quadrant, or other strategic locations.
- The new obstruction causes the guard’s movement to follow a repetitive pattern, effectively trapping them.

**Goal**: Determine the total number of distinct positions where a new obstruction can trap the guard in a loop.
