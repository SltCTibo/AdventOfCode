# Day09: Disk Fragmenter

## Part 1: Compacting Files Block by Block

### Disk Map Example:

`
2333133121414131402
`

This represents files and free spaces:

`
00…111…2…333.44.5555.6666.777.888899
`
### Compaction Process:
File blocks are moved one at a time from the end of the disk to the **leftmost free space**, filling gaps. For example, the disk map `12345` transforms as follows:

```
0..111….22222
02.111….2222.
022111….222..
0221112…22…
02211122..2….
022111222……
```

For the disk map `2333133121414131402`, the steps are more involved:

```
00…111…2…333.44.5555.6666.777.888899
009..111…2…333.44.5555.6666.777.88889.
0099.111…2…333.44.5555.6666.777.8888..
00998111…2…333.44.5555.6666.777.888…
```

### Filesystem Checksum:
After compaction, the checksum is calculated by summing the result of multiplying each block’s position by the file ID it contains, skipping free spaces. For example:
- Positions: `0, 1, 2, ...`
- File IDs: `0, 9, 9, ...`
- Calculation: `0 * 0 + 1 * 0 + 2 * 9 + ...`

In this example, the checksum is **1928**.

**Goal**: Calculate the resulting checksum after compacting the hard drive using this method.

---

## Part 2: Compacting Files Whole by Whole

After realizing the first method introduced performance issues, the amphipod proposes a new approach: compacting files by moving **whole files** instead of individual blocks.

### New Compaction Rules:
1. Move files to the **leftmost span of free space** that can fit the file.
2. Process files in **decreasing file ID order**.
3. If no free space to the left can fit the file, leave it in place.

### Example Process:
Using the same disk map `2333133121414131402`, the process differs:

```
00…111…2…333.44.5555.6666.777.888899
0099.111…2…333.44.5555.6666.777.8888..
0099.1117772…333.44.5555.6666…..8888..
0099.111777244.333….5555.6666…..8888..
00992111777.44.333….5555.6666…..8888..
```

### Updated Checksum:
The checksum is calculated the same way as in Part 1, based on the final disk layout. For this example, the checksum is **2858**.

**Goal**: Calculate the resulting checksum after compacting the hard drive using this updated method.