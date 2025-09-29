# fib (fib)

Count of bnuuies total following the Finobacci sequence.
- Solution for Rosalind's [Rabbits and Recurrence Relations](https://rosalind.info/problems/fib/).

Uses clap.rs.

# Usage

```
fib <N> <K>
```

## Arguments

```
  <N>  Months
  <K>  Number of bnuuy pairs per litter per month
```

## Options

``` 
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

### Output to file (sh)
```
$ cargo run -- $(cat test.txt) > test_output.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/fib 32 5`
```

### Output to Terminal
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
