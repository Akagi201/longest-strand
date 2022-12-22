# longest-strand

[![Rust CI](https://github.com/Akagi201/longest-strand/actions/workflows/rust.yml/badge.svg)](https://github.com/Akagi201/longest-strand/actions/workflows/rust.yml) [![Super Linter](https://github.com/Akagi201/longest-strand/actions/workflows/super_linter.yml/badge.svg)](https://github.com/Akagi201/longest-strand/actions/workflows/super_linter.yml)

Find the longest strand of bytes that is identical between two or more binary files

Use the test set attached (files sample.*)

The program should display:

- the length of the strand
- the file names where the largest strand appears
- the offset where the strand appears in each file

## Build && Run

```sh
cargo run --release # the release profile is optimized
```

## Solution

1. Open each file in binary mode and read the contents into memory.
2. Iterate through each file, comparing the contents to each other file, use Rayon to parallelize the comparisons.
3. For each comparison, find the longest common substring using a string comparison algorithm such as the Longest Common Subsequence (LCS) algorithm.
4. If a longer common substring is found, update the length of the longest strand and the file names and offsets where it appears.
5. Once all comparisons have been made, output the length of the longest strand, the file names where it appears, and the offsets where it appears in each file.

This solution has a time complexity of O(n^2 * m) where n is the number of files and m is the length of the longest file, so it may not be practical for very large files or a large number of files.
