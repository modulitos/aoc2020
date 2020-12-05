# Advent of Code 2020

This crate has some scaffolding so that all solutions will exist in a single crate, allowing us to re-use
implementations from previous days.

## Running a solutions:
To use the cli:

>cargo run <day> <part> <path to file>

eg:

>cargo run 1 2 src/exercises/day_01/receipts.txt


## Testing:

eg:
> cargo test day_01::solution

eg:
> cargo test day_01::solution::test_part_2_example
