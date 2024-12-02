# Day 02: Red-Nosed Reports

## Part 1: Reactor Safety Analysis

The engineers at the Red-Nosed reactor have tasked you with analyzing unusual data to identify safe reports. Each report consists of a sequence of levels, and a report is deemed **safe** if it meets both of these conditions:

1. Levels are either **strictly increasing** or **strictly decreasing**.
2. The difference between any two adjacent levels is at least 1 and at most 3.

### Example Analysis:
- `7 6 4 2 1`: **Safe** (strictly decreasing, differences within 1-3).
- `1 2 7 8 9`: **Unsafe** (difference of 5 between `2` and `7`).
- `9 7 6 2 1`: **Unsafe** (difference of 4 between `6` and `2`).
- `1 3 2 4 5`: **Unsafe** (switches from increasing to decreasing).
- `8 6 4 4 1`: **Unsafe** (repeated levels not allowed).
- `1 3 6 7 9`: **Safe** (strictly increasing, differences within 1-3).

### Goal:
Count the number of safe reports in the dataset.

---

## Part 2: Problem Dampener Assistance

The engineers realized they could use the **Problem Dampener**, a special module that allows the system to tolerate a single bad level in an otherwise safe report. By removing one problematic level, an unsafe report may become safe.

### Updated Example Analysis:
- `7 6 4 2 1`: **Safe** without removing any level.
- `1 2 7 8 9`: **Unsafe**, no single level removal makes it safe.
- `9 7 6 2 1`: **Unsafe**, no single level removal makes it safe.
- `1 3 2 4 5`: **Safe** by removing the second level (`3`).
- `8 6 4 4 1`: **Safe** by removing the third level (`4`).
- `1 3 6 7 9`: **Safe** without removing any level.

### Goal:
Re-evaluate the dataset considering the Problem Dampener and count the number of safe reports.
