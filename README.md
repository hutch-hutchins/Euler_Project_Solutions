# Project Euler in Rust

Rust solutions for [Project Euler](https://projecteuler.net/archives).

## Usage

Run one solved problem:

```powershell
cargo run -- 1
```

Run every solved problem:

```powershell
cargo run -- --all
```

Time one solved problem:

```powershell
cargo run --release -- 10 --time
```

Time every solved problem:

```powershell
cargo run --release -- --all --time
```

Run the regression tests:

```powershell
cargo test
```

## Structure

Each problem lives in `src/problems/` with a zero-padded filename such as `p0001.rs`.
Problem modules expose `solve() -> String`; printing, argument parsing, and timing stay in `src/main.rs`.

Shared helpers live in `src/util/` after repetition makes them worthwhile.

## Data Files

Some Project Euler problems use official text inputs. Store those files raw in `data/` and name them by problem number, such as `p0022_names.txt`.
Problem modules should load fixed inputs with `include_str!()` so `cargo test` and `cargo run` work from the repo root without extra path arguments.

## Progress

| Problem | Title | Status | Notes |
|---:|---|---|---|
| 1 | Multiples of 3 or 5 | solved | Direct iteration; arithmetic-series version is possible. |
| 2 | Even Fibonacci Numbers | solved | Iterates Fibonacci terms below four million and sums the even terms. |
| 3 | Largest Prime Factor | solved | Trial division with factor removal. |
| 4 | Largest Palindrome Product | solved | Descending product search with numeric palindrome check. |
| 5 | Smallest Multiple | solved | Folds least common multiple across `1..=20`. |
| 6 | Sum Square Difference | solved | Closed-form sums. |
| 7 | 10 001st Prime | solved | Reuses trial-division primality helper. |
| 8 | Largest Product in a Series | solved | Embeds the 1000-digit series and scans fixed windows. |
| 9 | Special Pythagorean Triplet | solved | Searches ordered triples for the target sum. |
| 10 | Summation of Primes | solved | Sieve of Eratosthenes below two million. |
