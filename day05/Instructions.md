# Day05: Safety Manual Updates

## Part 1: Identifying Correctly-Ordered Updates

The Elves at the North Pole printing department need to ensure that safety manual pages are printed in a specific order. The page ordering rules, provided as `X|Y`, mean that if both pages `X` and `Y` are part of the update, page `X` must appear before page `Y`. Each update contains a list of page numbers, and the task is to determine which updates are already in the correct order.

```
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
```

For example:
- The update `75,47,61,53,29` is in the correct order because all ordering rules are satisfied.
- The update `75,97,47,61,53` is **not** in the correct order because it violates the rule `97|75`.

After identifying the correctly-ordered updates, the Elves also need to know the middle page number for each of them. For example:
- For the update `75,47,61,53,29`, the middle page is `61`.
- Summing the middle pages of all correctly-ordered updates provides the final result for Part 1.

**Goal**: Find the sum of the middle page numbers for all correctly-ordered updates.

---

## Part 2: Fixing Incorrectly-Ordered Updates

For updates that are **not** in the correct order, the task is to reorder them based on the provided page ordering rules. Once reordered, calculate the middle page number for each fixed update.

For example:
- The incorrect update `75,97,47,61,53` becomes `97,75,47,61,53` after reordering.
- The incorrect update `61,13,29` becomes `61,29,13` after reordering.

After reordering, the middle page numbers of these updates are summed to provide the final result for Part 2.

**Goal**: Find the sum of the middle page numbers for all fixed updates.

