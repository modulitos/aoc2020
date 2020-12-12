# Advent of Code 2020

This crate has some scaffolding so that all solutions will exist in a single crate, allowing us to re-use
implementations from previous days.

## Running a solutions:
**NOTE: this project only runs on the Rust Nightly toolchain**

To use the cli:

> cargo run `<day>` `<part>` `<path to file>`

where `day` is [1-25], `part` is [1-2], and `path to file` is the relative path to the input file.

eg:

>cargo run 1 2 src/exercises/day_01/receipts.txt

will output the solution to Day 1, Part 2, using `src/exercises/day_01/receipts.txt` as the input.

## Testing:

eg:
> cargo test day_01::solution

eg:
> cargo test day_01::solution::test_part_2_example


## lessons learned

Day 5: Leveraging trait inheritance, and specifying the associated types in the inherited trait.
Day 6: Using `.fold_first`, which is only available on Rust nightly.