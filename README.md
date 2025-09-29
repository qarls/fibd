# fib

Count of bnuuies total following the Finobacci sequence.
- Solution for Rosalind's [Rabbits and Recurrence Relations](https://rosalind.info/problems/fib/).

Uses clap.rs.

# Usage

```
fib <N> <K>
```

Where...
- N: number of months or length of Fibonacci sequence.
- K: number of bnuuy pairs in litter; or a coefficient applied to the addend in Finobacci Seqeuence.

## Examples

### Output to file (sh)
Works in Linux terminals.
```
$ cargo run -- $(cat test.txt) > test_output.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/fib 32 5`
```

### Output to Terminal
To directly input and receive values, feel free to simply input them as separate args.
```
$ cargo run -- 5 3
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/fib 5 3`
19
```

# Updates and Comments

## [2025-09-28]
I was considering using a vector of strings and indexing--though that would push Rust to use the heap;
it would also mean cloning doubling the capacity at powers of two.
- Bnuuy
