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
| 11 | Largest Product in a Grid | solved | Parses the grid and scans four directions. |
| 12 | Highly Divisible Triangular Number | solved | Uses divisor counts from prime factorization. |
| 13 | Large Sum | solved | Uses decimal-vector addition for 50-digit values. |
| 14 | Longest Collatz Sequence | solved | Memoizes Collatz chain lengths. |
| 15 | Lattice Paths | solved | Computes the central binomial coefficient. |
| 16 | Power Digit Sum | solved | Uses decimal-vector multiplication. |
| 17 | Number Letter Counts | solved | Counts British-usage number words without spaces or hyphens. |
| 18 | Maximum Path Sum I | solved | Bottom-up dynamic programming over the triangle. |
| 19 | Counting Sundays | solved | Simulates month starts using Gregorian leap-year rules. |
| 20 | Factorial Digit Sum | solved | Reuses decimal-vector multiplication. |
| 21 | Amicable Numbers | solved | Uses sieved proper-divisor sums. |
| 22 | Names Scores | solved | Uses `data/p0022_names.txt` and alphabetical scoring. |
| 23 | Non-Abundant Sums | solved | Marks sums of abundant-number pairs. |
| 24 | Lexicographic Permutations | solved | Uses factorial block selection. |
| 25 | 1000-Digit Fibonacci Number | solved | Reuses decimal-vector addition. |
| 26 | Reciprocal Cycles | solved | Tracks first-seen remainders during long division. |
| 27 | Quadratic Primes | solved | Searches coefficient pairs by prime streak length. |
| 28 | Number Spiral Diagonals | solved | Applies the corner-sum formula for each layer. |
| 29 | Distinct Powers | solved | Uses decimal-vector powers stored as strings. |
| 30 | Digit Fifth Powers | solved | Scans to the digit-power upper bound. |
| 31 | Coin Sums | solved | Dynamic programming over UK coin denominations. |
| 32 | Pandigital Products | solved | Finds 1-9 pandigital identities and deduplicates products. |
| 33 | Digit Cancelling Fractions | solved | Brute-forces two-digit fractions and reduces the product. |
| 34 | Digit Factorials | solved | Precomputes digit factorials and scans to a bounded limit. |
| 35 | Circular Primes | solved | Uses a sieve plus decimal rotations. |
| 36 | Double-base Palindromes | solved | Reuses base-aware palindrome checks. |
| 37 | Truncatable Primes | solved | Tests prime status after left and right truncations. |
| 38 | Pandigital Multiples | solved | Searches concatenated products for 1-9 pandigital values. |
| 39 | Integer Right Triangles | solved | Counts Pythagorean triples by perimeter. |
| 40 | Champernowne's Constant | solved | Computes requested sequence digits by index. |
| 41 | Pandigital Prime | solved | Permutes pandigital digits and tests primality. |
| 42 | Coded Triangle Numbers | solved | Uses `data/p0042_words.txt` and triangle-number tests. |
| 43 | Sub-string Divisibility | solved | Builds 0-9 pandigital numbers with divisibility pruning. |
| 44 | Pentagon Numbers | solved | Searches pentagonal pairs with pentagonal sum and difference. |
| 45 | Triangular, Pentagonal, and Hexagonal | solved | Scans hexagonal numbers and tests pentagonal membership. |
| 46 | Goldbach's Other Conjecture | solved | Tests odd composites against prime plus twice-square forms. |
| 47 | Distinct Primes Factors | solved | Uses a distinct-prime-factor count sieve. |
| 48 | Self Powers | solved | Computes the modular sum of self powers. |
| 49 | Prime Permutations | solved | Groups four-digit primes by digit signature. |
| 50 | Consecutive Prime Sum | solved | Uses prime prefix sums below one million. |
| 51 | Prime Digit Replacements | solved | Searches repeated-digit masks across primes. |
| 52 | Permuted Multiples | solved | Compares digit signatures for multiples two through six. |
| 53 | Combinatoric Selections | solved | Counts binomial coefficients above one million. |
| 54 | Poker Hands | solved | Uses `data/p0054_poker.txt` and a local hand evaluator. |
| 55 | Lychrel Numbers | solved | Uses decimal reverse-add iteration. |
| 56 | Powerful Digit Sum | solved | Reuses decimal-vector multiplication. |
| 57 | Square Root Convergents | solved | Tracks big numerator and denominator decimal vectors. |
| 58 | Spiral Primes | solved | Tests spiral corners until the prime ratio drops below ten percent. |
| 59 | XOR Decryption | solved | Uses `data/p0059_cipher.txt` and scores three-letter lowercase keys. |
| 60 | Prime Pair Sets | solved | Backtracks over compatible concatenating-prime sets. |
