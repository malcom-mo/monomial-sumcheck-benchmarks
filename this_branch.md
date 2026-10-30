# Comparing optimized old and new kernels under explicit claim assumptions

Changes relative to `origin/main` at `0a81dad`:
- optimize existing kernels so they reuse weighted inputs,
- compare every kernel in explicit claim-aware (only compute 2 quantities)
  and claim-unaware (compute three quantities) mode,
- add flipped and different-terminal-claim/preweighted kernels.

In aware mode, flipping saves two additions per loop iteration relative to `projective`
(but needs commitment support for flipped evaluations).

Preweighting also saves one multiplication in weighted cases but changes the final claims.

## Implemented arithmetic counts

Let $N=2^n$. M, A and S denote logical field multiplications, additions and subtractions.
Counts include evaluation and binding.

| Kernel | Aware M | Aware A | Unaware M | Unaware A | S in either mode |
|---|---:|---:|---:|---:|---:|
| Unweighted Boolean | 4N-4 | 4N-4 | 5N-5 | 5N-5 | 4N-4 |
| Unweighted projective | 4N-4 | 6N-6 | 5N-5 | 7N-7 | 0 |
| Unweighted flipped | 4N-4 | **4N-4** | 5N-5 | 7N-7 | 0 |
| Weighted Boolean | 6N-6 | 4N-4 | 7N-7 | 5N-5 | 4N-4 |
| Weighted projective | 6N-6 | 6N-6 | 7N-7 | 7N-7 | 0 |
| Projective different claims | **5N-4** | 6N-6 | **6N-5** | 7N-7 | 0 |
| Flipped different claims | **5N-4** | **4N-4** | **6N-5** | 7N-7 | 0 |

Relative to ordinary aware weighted projective, both preweighted variants save $N-2$ multiplications.
Flipped additionally saves $2N-2$ additions.
In unaware mode, flipped and projective have the same addition counts.

## Sample results for GF128, claim-aware, $n=20$

In the `sumcheck` benchmark, compared to `proj_delayed`, the `flipped_delayed`
protocols saved **0.637 ms (9.78%)** in the unweighted and **1.708 ms (18.88%)** in the weighted case.

Results for all fields and modes on an Apple M1 Pro are in `artifacts/data/m1pro-runs`.

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 7.586 ms | [7.578, 7.596] ms | 7.578 ms | 7.595 ms |
| Unweighted | projective | 20 | 5 | 7.477 ms | [7.470, 7.526] ms | 7.470 ms | 7.524 ms |
| Unweighted | flipped | 20 | 5 | 7.370 ms | [7.366, 7.384] ms | 7.366 ms | 7.383 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 6.671 ms | [6.629, 6.746] ms | 6.629 ms | 6.743 ms |
| Unweighted | projective (delayed) | 20 | 5 | 6.513 ms | [6.509, 6.526] ms | 6.509 ms | 6.526 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **5.876 ms** | [5.864, 5.932] ms | 5.865 ms | 5.930 ms |
| Weighted | boolean | 20 | 5 | 10.456 ms | [10.433, 10.468] ms | 10.434 ms | 10.468 ms |
| Weighted | projective | 20 | 5 | 10.347 ms | [10.337, 10.413] ms | 10.337 ms | 10.411 ms |
| Weighted | projective-different-claims | 20 | 5 | 8.942 ms | [8.927, 8.943] ms | 8.927 ms | 8.943 ms |
| Weighted | flipped-different-claims | 20 | 5 | 8.795 ms | [8.784, 8.823] ms | 8.784 ms | 8.822 ms |
| Weighted | boolean (delayed) | 20 | 5 | 9.158 ms | [9.151, 9.166] ms | 9.151 ms | 9.166 ms |
| Weighted | projective (delayed) | 20 | 5 | 9.048 ms | [9.042, 9.058] ms | 9.042 ms | 9.057 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 7.735 ms | [7.725, 7.743] ms | 7.725 ms | 7.742 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **7.340 ms** | [7.330, 7.343] ms | 7.330 ms | 7.343 ms |

## Added protocols and final claims

Write multilinear polynomials in the monomial basis as
$f(x)=\sum_{i\in\{0,1\}^n}f[i]x^i$, where
$a^i=\prod_k a_k^{i_k}$. Brackets denote coefficients;
parentheses denote evaluations. In a round, $j$ indexes the remaining
variables and $f[0,j],f[1,j]$ split the current coefficient table.

### `flipped` (unweighted)

For the claim $t=\sum_i f[i]g[i]$, define the round polynomial as

$$
p(y)=\sum_j (f[0,j]+y f[1,j])(g[1,j]+y g[0,j]).
$$

The linear coefficient is
$p[1]=\sum_j(f[0,j]g[0,j]+f[1,j]g[1,j])=t$.
The claim-aware prover sends

$$
p[0]=\sum_j f[0,j]g[1,j],\qquad
p[2]=\sum_j f[1,j]g[0,j].
$$

The verifier reconstructs $p$ using $p[1]=t$ and samples a challenge $r$.
Bind the tables as

$$
f'[j]=f[0,j]+r f[1,j],\qquad
g'[j]=g[1,j]+r g[0,j].
$$

The next claim is $t'=p(r)=\sum_j f'[j]g'[j]$. Computing the two sent
coefficients takes two multiplications and two accumulator additions per
pair; binding takes another two multiplications and two additions. The total
is four multiplications and four additions per pair, versus four
multiplications and six additions for claim-aware unweighted projective.

After n rounds the terminal values are $f(r)$ and

$$
g_{\mathrm{rev}}(r)=\sum_i g[i] \prod_k r_k^{1-i_k}.
$$

This polynomial definition also handles zero challenges. If every $r_k$ is
nonzero, it equals $(\prod_k r_k)g(1/r_1,\ldots,1/r_n)$.

### `flipped-different-claims`, `projective-different-claims` (weighted)

For the claim $t=\sum_i a^i f[i]g[i]$, the weighted kernels set
$F[i]=a^i f[i]$ during the first evaluation pass, store these coefficients,
and then use unweighted rounds on $F$ and $g$.

For `flipped-different-claims`, use the flipped polynomial above with $F$ in place of $f$.
The terminal values are $F(r)=f(a_1r_1,\ldots,a_nr_n)$ and $g_{\mathrm{rev}}(r)$.

For `projective-different-claims`, use the ordinary projective round polynomial

$$
h(y)=\sum_j(F[0,j]+yF[1,j])(g[0,j]+yg[1,j]).
$$

with $t=h[0]+h[2]$.
The terminal values are $f(a_1r_1,\ldots,a_nr_n)$ and $g(r)$.

These variants thus need commitment-opening support for their changed final claims.

---

Side note: Cases like $\sum_i a^i(f[i]g[i]-h[i])$, which are not in these benches, seem impossible with the projective protocol as described in the paper.
However, a ProtoStar-style adjustment could be made to also handle those -- sketched [elsewhere](https://mal.com.de/monomial-sumcheck).
