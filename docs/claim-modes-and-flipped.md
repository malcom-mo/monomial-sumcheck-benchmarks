# Claim modes and flipped benchmark kernels

Every sumcheck case has explicit `-claim-aware` and `-claim-unaware` benchmark
IDs on this branch. Both modes share inputs, challenges, suffix weights and
backend choices. Awareness is a const generic specialized at compilation;
there is no runtime awareness switch in a timed loop.

A claim-aware kernel computes two round quantities, assuming the incoming
claim provides the relation needed for the omitted quantity. A claim-unaware
kernel computes three independently. The kernels measure these arithmetic
loops with precomputed challenges; they do not implement claim reconstruction,
transcript hashing or polynomial-commitment verification.

| Kernel | Aware quantities | Unaware quantities |
|---|---|---|
| Unweighted Boolean | h0, h_inf | h0, h1, h_inf |
| Unweighted projective | h1, h_inf | h0, h1, h_inf |
| Weighted Boolean/projective split-equality | q1, q_inf | q0, q1, q_inf |
| Fp128 {1,inf}, weighted or unweighted | value at 0, leading coefficient | values at 0 and 1, leading coefficient |
| Projective different claims | h1, h_inf | h0, h1, h_inf |
| Flipped, including different claims | c0, c2 | p(0), p(1), p[2] |

For ordinary unweighted Boolean, the claim is h0+h1; for projective it is
h0+h_inf. For flipped it is the linear coefficient c1. Thus aware flipped
evaluates only c0 and c2, while unaware flipped also evaluates p(1), from which
c1 could be recovered as p(1)-c0-c2. Weighted split-equality keeps its existing
claim assumptions. Claim reconstruction costs are outside these benchmarks.

The existing unsuffixed Rust functions retain their previous defaults:
unweighted functions are unaware, weighted functions are aware. Explicit
`_claim_aware` and `_claim_unaware` Rust entry points are available for both.
The packed BB5 evaluation helper APIs retain their original two-value results.
The benchmark IDs, however, always spell out the mode. Historical unsuffixed
benchmark IDs are replaced rather than registered as duplicate measurements;
historical paper artifacts are unchanged.

## Flipped polynomials and changed claims

Unweighted flipped evaluates

```text
p(y) = sum_j (f0 + y*f1) (g1 + y*g0)
```

and binds f to `f0+r*f1`, g to `g1+r*g0`. Its terminal values are f(r) and
g_rev(r), where

```text
g_rev(r) = sum_i g[i] product_{k:i_k=0} r[k].
```

This definition handles zero challenges. With all challenges nonzero it also
equals `product_k r[k] * g(1/r)`.

Weighted `projective-different-claims` and `flipped-different-claims` absorb
full monomial weights W[i] into f during the first evaluation pass, store
those weighted coefficients, and use unweighted binding thereafter. Every
round preserves the selected claim mode. Projective finishes at f(a*r), g(r);
flipped finishes at f(a*r), g_rev(r). These opening points differ from those
of ordinary weighted projective, so a commitment protocol must support the
changed terminal claims.

Preweighting is timed. It adds N coefficient stores in the first evaluation
pass, without a separate pass or polynomial allocation. Suffix construction
and input restoration stay outside timing. No separate weight-point argument
is needed. There is no direct weighted flipped or folded-preweighting case
on this branch; the weighted flipped comparison uses stored preweighting.

## Arithmetic in the current implementations

Let N=2^n, n>=1. Counts include evaluation and binding. A/S count
additions/subtractions; fused multiply-add is one multiplication plus one
addition. These are implemented logical counts, not proposed minima.

| Kernel | Aware M | Aware A | Unaware M | Unaware A | S in either mode |
|---|---:|---:|---:|---:|---:|
| Unweighted Boolean | 4N-4 | 4N-4 | 5N-5 | 5N-5 | 4N-4 |
| Unweighted projective | 4N-4 | 6N-6 | 5N-5 | 7N-7 | 0 |
| Unweighted flipped | 4N-4 | **4N-4** | 5N-5 | 7N-7 | 0 |
| Weighted Boolean | 6N-6 | 4N-4 | 7N-7 | 5N-5 | 4N-4 |
| Weighted projective | 6N-6 | 6N-6 | 7N-7 | 7N-7 | 0 |
| Projective different claims | **5N-4** | 6N-6 | **6N-5** | 7N-7 | 0 |
| Flipped different claims | **5N-4** | **4N-4** | **6N-5** | 7N-7 | 0 |

For n=0 all kernels do no work. Unit-weight multiplications and initial
accumulator additions are not special-cased. Neither are Boolean differences
cached across the message boundary. Fp128 {1,inf} has its additional basis
conversion subtractions; it is tested and benchmarked in both modes but is
outside this table. Eager and delayed versions share the listed logical
counts; backend reduction, multiplier setup and packed horizontal sums are
excluded. Fewer field operations do not establish a runtime improvement.

## Coverage and reproduction

The eager and delayed variants cover BN254, branch-free BN254, upper-limb
BN254, Fp128, BB4, BB5 and KB5, and GF(2^128). BB5 retains packed weighted
evaluation with scalar tails. BN254 upper-limb bindings retain sparse
multiplication for both polynomials, including after preweighting. The GF
Karatsuba experiment stays on its separate branch.

All compared modes and kernels share an instance per field/size. Registration
order is randomized; exact-case process repetitions use the existing collector.
Every callback exposes both final polynomial values through an optimizer
barrier. Independent direct-contraction tests check all round quantities and
both terminal values across the field/backend/mode matrix, including empty
and one-variable inputs and zero/unit weights and challenges. Counted-field
tests check the generic arithmetic formulas.

Run an exploratory n=20 suite:

```sh
cargo bench --bench sumcheck -- '/20$'
```

Compare weighted BN254 delayed kernels in one claim mode:

```sh
python3 scripts/collect.py --suite sumcheck \
  --case sumcheck_deg2_eq/BN254/proj_delayed-claim-aware/20 \
  --case sumcheck_deg2_eq/BN254/projective-different-claims-delayed-claim-aware/20 \
  --case sumcheck_deg2_eq/BN254/flipped-different-claims-delayed-claim-aware/20 \
  --repetitions 10 --max-load-per-cpu 0.4 \
  --label weighted-bn254-aware-n20
```

Replace `claim-aware` with `claim-unaware` to compare the other mode. To
isolate awareness itself, pair the aware/unaware IDs of the same kernel. Use
a clean committed checkout and follow [benchmark-methodology.md](benchmark-methodology.md)
for manifests, raw samples and paired summaries. Earlier measurements use
other source revisions and claim assumptions; new performance claims need
fresh process repetitions.
