use super::*;

/// Boolean: evaluations on {0,1}^n.
/// Evaluate: 3 mul, 3 add, 2 sub per pair.
/// Bind:     2 mul, 2 add, 2 sub per pair.
pub fn sumcheck_deg2_boolean<F>(f: &mut Vec<F>, g: &mut Vec<F>, challenges: &[F], zero: F)
where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h0 = zero;
        let mut h1 = zero;
        let mut h_inf = zero;

        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];

            let df = f1 - f0;
            let dg = g1 - g0;

            h0 += f0 * g0;
            h1 += f1 * g1;
            h_inf += df * dg;
        }

        round_message((h0, h1, h_inf));

        let r = challenges[round];

        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];

            f[j] = f0 + r * (f1 - f0);
            g[j] = g0 + r * (g1 - g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

/// Projective: monomial coefficients on {0,inf}^n.
/// Evaluate: 3 mul, 5 add per pair.
/// Bind:     2 mul, 2 add per pair.
pub fn sumcheck_deg2_projective<F>(f: &mut Vec<F>, g: &mut Vec<F>, challenges: &[F], zero: F)
where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h0 = zero;
        let mut h1 = zero;
        let mut h_inf = zero;

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            h0 += f0 * g0;
            h_inf += fi * gi;
            h1 += (f0 + fi) * (g0 + gi);
        }

        round_message((h0, h1, h_inf));

        let r = challenges[round];

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            f[j] = f0 + r * fi;
            g[j] = g0 + r * gi;
        }

        f.truncate(half);
        g.truncate(half);
    }
}

/// Gruen split-eq on Boolean basis.
/// q1 and q_inf each cost one mul by eq_rest per pair.
pub fn sumcheck_deg2_eq_gruen_boolean<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];

        let mut q1 = zero;
        let mut q_inf = zero;

        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            let ew = eq_rest[j];
            let wf0 = f0 * ew;
            let wf1 = f1 * ew;
            let df = wf1 - wf0;
            let dg = g1 - g0;

            q1 += wf1 * g1;
            q_inf += df * dg;
        }

        round_message((q1, q_inf));

        let r = challenges[round];
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];

            f[j] = f0 + r * (f1 - f0);
            g[j] = g0 + r * (g1 - g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

/// Gruen split-eq on projective basis.
pub fn sumcheck_deg2_eq_gruen_projective<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];

        let mut q1 = zero;
        let mut q_inf = zero;

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            let ew = eq_rest[j];
            let wf0 = f0 * ew;
            let wfi = fi * ew;

            q1 += (wf0 + wfi) * (g0 + gi);
            q_inf += wfi * gi;
        }

        round_message((q1, q_inf));

        let r = challenges[round];
        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            f[j] = f0 + r * fi;
            g[j] = g0 + r * gi;
        }

        f.truncate(half);
        g.truncate(half);
    }
}

/// Preweight and store f during the first evaluation pass. Every round emits
/// (h1,h_inf); the claim supplies h0. Terminal values are f(a*r) and g(r).
/// Costs N+4(N-1) multiplications and 6(N-1) additions, for N>=2.
pub fn sumcheck_deg2_eq_projective_different_claims<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    if challenges.is_empty() {
        return;
    }
    let half = f.len() / 2;
    let weights = &suffix_eq[0];
    let mut h1 = zero;
    let mut h_inf = zero;
    for j in 0..half {
        let f0 = f[2 * j] * weights[2 * j];
        let f1 = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let g1 = g[2 * j + 1];
        h1 += (f0 + f1) * (g0 + g1);
        h_inf += f1 * g1;
        f[2 * j] = f0;
        f[2 * j + 1] = f1;
    }
    round_message((h1, h_inf));
    let r = challenges[0];
    for j in 0..half {
        let f0 = f[2 * j];
        let f1 = f[2 * j + 1];
        let g0 = g[2 * j];
        let g1 = g[2 * j + 1];
        f[j] = f0 + r * f1;
        g[j] = g0 + r * g1;
    }
    f.truncate(half);
    g.truncate(half);
    projective_different_claims_tail(f, g, &challenges[1..], zero);
}

// Two-value projective evaluation after all monomial weights are absorbed.
pub(super) fn projective_different_claims_tail<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    for &r in challenges {
        let half = f.len() / 2;
        let mut h1 = zero;
        let mut h_inf = zero;
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            h1 += (f0 + f1) * (g0 + g1);
            h_inf += f1 * g1;
        }
        round_message((h1, h_inf));
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            f[j] = f0 + r * f1;
            g[j] = g0 + r * g1;
        }
        f.truncate(half);
        g.truncate(half);
    }
}
