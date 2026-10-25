# Projective sumcheck with different terminal claims

The weighted `projective-different-claims` and
`projective-different-claims-delayed` cases preweight f during the first
round's evaluation pass, store those weighted coefficients, and continue
with ordinary projective binding. They compute two values per round:
`h1` and `h_inf`. The incoming claim supplies `h0 = claim - h_inf`.
The original unweighted benchmarks still compute three values independently;
these weighted variants use their own two-value projective loops.

## Polynomial and terminal claims

For the initial weighted claim

```text
t = sum_i W[i] f[i] g[i],    W[i] = product_{k:i_k=1} a[k],
```

set `F[i] = W[i] f[i]`. In every round the new protocol evaluates

```text
h(y) = sum_j (F[2j] + y F[2j+1]) (g[2j] + y g[2j+1]).
```

It computes `h(1)` and the quadratic coefficient `h_inf`; the dot-product
claim is `h(0) + h_inf`. After emitting those values it binds both tables
with the same challenge r. The terminal values are `F(r) = f(a*r)` and `g(r)`,
where a*r is coordinatewise multiplication. Ordinary weighted projective
sumcheck instead binds the original f and g and retains the remaining weights
in the recursive claim, terminating at `f(r)` and `g(r)`. Thus the new variant
changes the round polynomial and the terminal opening point for f.

These are evaluation-and-binding kernels with precomputed challenges. A
commitment protocol using them must support the changed opening point for f;
transcripts and commitment openings are outside this benchmark suite.

## First-pass conversion and arithmetic

The first evaluation loads each original f coefficient, multiplies it by its
full monomial weight from `suffix_eq[0]`, reuses it in both requested products,
and stores it back. The first binding and every later binding use r for both
polynomials. There is no separate preweighting pass or polynomial allocation.
The extra N coefficient stores occur during the first evaluation pass.
Preweighting is included in the timed routine. Suffix-table construction and
input restoration remain outside timing. The interface needs no separate
weight-point parameter.

For N=2^n and n>=1, with P=N-1 pairs across all rounds:

| Kernel | Multiplications | Additions |
|---|---:|---:|
| Ordinary weighted projective | 6P | 6P |
| Projective different claims | N+4P = 5N-4 | 6P |

The new kernel performs N preweighting multiplications plus two products and
two binding multiplications per pair. It saves N-2 multiplications overall:
the counts tie at N=2 and approach a one-sixth saving for large N. Unit weights
are not special-cased. At n=0 all kernels do no work. Eager and delayed variants
have the same logical field-operation counts. The counts exclude backend
reduction internals, multiplication-helper setup, packed horizontal sums,
claim reconstruction and transcript work; fused multiply-add represents one
multiplication plus one addition. An instrumented field verifies the new
formula, including the first round and the empty-challenge case.

## Implementations and validation

Both variants are registered for BN254, BN254 branch-free, BN254 upper-limb,
Fp128, BB4, BB5, KB5 and GF(2^128). BB5 retains packed first-round evaluation
with scalar tails. The delayed variants reuse the existing accumulation and
constant-multiplier backends. BN254 upper-limb binding retains sparse challenge
multiplication for both polynomials from the first round onward.

Tests check each round against independent contraction of the original inputs
and compare specialized implementations with the generic kernel. They cover
empty and one-variable inputs, random/zero/unit/mixed weights, and random/zero/
unit challenges. Both final polynomial values are checked. Benchmark callbacks
expose both final values through an optimizer barrier for all compared cases.

## Reproduction

A quick exploratory n=20 comparison of all weighted projective cases:

```sh
cargo bench --bench sumcheck -- 'sumcheck_deg2_eq/.*/(projective|proj_delayed|projective-different-claims|projective-different-claims-delayed)/20$'
```

For independent paired process repetitions, from a clean committed tree:

```sh
python3 scripts/collect.py --suite sumcheck \
  --case sumcheck_deg2_eq/BN254/proj_delayed/20 \
  --case sumcheck_deg2_eq/BN254/projective-different-claims-delayed/20 \
  --repetitions 10 --max-load-per-cpu 0.4 \
  --label projective-different-claims-bn254-n20
```

See [benchmark-methodology.md](benchmark-methodology.md) for raw artifacts and
paired summaries. The existing-kernel input-reuse optimization and this new
variant require fresh timings; historical paper data are left unchanged.
