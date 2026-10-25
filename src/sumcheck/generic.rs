use super::*;

/// Boolean: evaluations on {0,1}^n.
/// Unaware evaluation: 3 mul, 3 add, 2 sub per pair; aware omits h1.
/// Bind:     2 mul, 2 add, 2 sub per pair.
pub fn sumcheck_deg2_boolean_with_claim_mode<const CLAIM_AWARE: bool, F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
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
            if !CLAIM_AWARE {
                h1 += f1 * g1;
            }
            h_inf += df * dg;
        }

        if CLAIM_AWARE {
            round_message((h0, h_inf));
        } else {
            round_message((h0, h1, h_inf));
        }

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

pub fn sumcheck_deg2_boolean<F>(f: &mut Vec<F>, g: &mut Vec<F>, challenges: &[F], zero: F)
where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    sumcheck_deg2_boolean_with_claim_mode::<false, _>(f, g, challenges, zero);
}

pub fn sumcheck_deg2_boolean_claim_aware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    sumcheck_deg2_boolean_with_claim_mode::<true, _>(f, g, challenges, zero);
}

pub fn sumcheck_deg2_boolean_claim_unaware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    sumcheck_deg2_boolean_with_claim_mode::<false, _>(f, g, challenges, zero);
}

/// Projective: monomial coefficients on {0,inf}^n.
/// Unaware evaluation: 3 mul, 5 add per pair; aware omits h0.
/// Bind:     2 mul, 2 add per pair.
pub fn sumcheck_deg2_projective_with_claim_mode<const CLAIM_AWARE: bool, F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
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

            if !CLAIM_AWARE {
                h0 += f0 * g0;
            }
            h_inf += fi * gi;
            h1 += (f0 + fi) * (g0 + gi);
        }

        if CLAIM_AWARE {
            round_message((h1, h_inf));
        } else {
            round_message((h0, h1, h_inf));
        }

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

pub fn sumcheck_deg2_projective<F>(f: &mut Vec<F>, g: &mut Vec<F>, challenges: &[F], zero: F)
where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_projective_with_claim_mode::<false, _>(f, g, challenges, zero);
}

pub fn sumcheck_deg2_projective_claim_aware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_projective_with_claim_mode::<true, _>(f, g, challenges, zero);
}

pub fn sumcheck_deg2_projective_claim_unaware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_projective_with_claim_mode::<false, _>(f, g, challenges, zero);
}

/// Gruen split-eq on Boolean basis.
/// q1 and q_inf each cost one mul by eq_rest per pair.
pub fn sumcheck_deg2_eq_gruen_boolean_with_claim_mode<const CLAIM_AWARE: bool, F>(
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
        let mut q0 = zero;
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

            if !CLAIM_AWARE {
                q0 += wf0 * g0;
            }
            q1 += wf1 * g1;
            q_inf += df * dg;
        }

        if CLAIM_AWARE {
            round_message((q1, q_inf));
        } else {
            round_message((q0, q1, q_inf));
        }

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

pub fn sumcheck_deg2_eq_gruen_boolean<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_gruen_boolean_with_claim_mode::<true, _>(f, g, suffix_eq, challenges, zero);
}

pub fn sumcheck_deg2_eq_gruen_boolean_claim_aware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_gruen_boolean_with_claim_mode::<true, _>(f, g, suffix_eq, challenges, zero);
}

pub fn sumcheck_deg2_eq_gruen_boolean_claim_unaware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Sub<Output = F> + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_gruen_boolean_with_claim_mode::<false, _>(f, g, suffix_eq, challenges, zero);
}

/// Gruen split-eq on projective basis.
pub fn sumcheck_deg2_eq_gruen_projective_with_claim_mode<const CLAIM_AWARE: bool, F>(
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
        let mut q0 = zero;
        let mut q_inf = zero;

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            let ew = eq_rest[j];
            let wf0 = f0 * ew;
            let wfi = fi * ew;

            if !CLAIM_AWARE {
                q0 += wf0 * g0;
            }
            q1 += (wf0 + wfi) * (g0 + gi);
            q_inf += wfi * gi;
        }

        if CLAIM_AWARE {
            round_message((q1, q_inf));
        } else {
            round_message((q0, q1, q_inf));
        }

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

pub fn sumcheck_deg2_eq_gruen_projective<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_gruen_projective_with_claim_mode::<true, _>(f, g, suffix_eq, challenges, zero);
}

pub fn sumcheck_deg2_eq_gruen_projective_claim_aware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_gruen_projective_with_claim_mode::<true, _>(f, g, suffix_eq, challenges, zero);
}

pub fn sumcheck_deg2_eq_gruen_projective_claim_unaware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_gruen_projective_with_claim_mode::<false, _>(
        f, g, suffix_eq, challenges, zero,
    );
}

/// Preweight and store f during the first evaluation pass. Aware rounds emit
/// (h1,h_inf); unaware rounds also compute h0. Terminal values: f(a*r), g(r).
/// For N>=2, aware costs 5N-4 M and 6(N-1) A; unaware 6N-5 M and 7(N-1) A.
pub fn sumcheck_deg2_eq_projective_different_claims_with_claim_mode<const CLAIM_AWARE: bool, F>(
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
    let mut h0 = zero;
    let mut h1 = zero;
    let mut h_inf = zero;
    for j in 0..half {
        let f0 = f[2 * j] * weights[2 * j];
        let f1 = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let g1 = g[2 * j + 1];
        if !CLAIM_AWARE {
            h0 += f0 * g0;
        }
        h1 += (f0 + f1) * (g0 + g1);
        h_inf += f1 * g1;
        f[2 * j] = f0;
        f[2 * j + 1] = f1;
    }
    if CLAIM_AWARE {
        round_message((h1, h_inf));
    } else {
        round_message((h0, h1, h_inf));
    }
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
    projective_different_claims_tail_with_claim_mode::<CLAIM_AWARE, _>(
        f,
        g,
        &challenges[1..],
        zero,
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_projective_different_claims_with_claim_mode::<true, _>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims_claim_aware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_projective_different_claims_with_claim_mode::<true, _>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims_claim_unaware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_projective_different_claims_with_claim_mode::<false, _>(
        f, g, suffix_eq, challenges, zero,
    );
}

// Two-value projective evaluation after all monomial weights are absorbed.
pub(super) fn projective_different_claims_tail_with_claim_mode<const CLAIM_AWARE: bool, F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    for &r in challenges {
        let half = f.len() / 2;
        let mut h0 = zero;
        let mut h1 = zero;
        let mut h_inf = zero;
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            if !CLAIM_AWARE {
                h0 += f0 * g0;
            }
            h1 += (f0 + f1) * (g0 + g1);
            h_inf += f1 * g1;
        }
        if CLAIM_AWARE {
            round_message((h1, h_inf));
        } else {
            round_message((h0, h1, h_inf));
        }
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

/// Flipped binding ends at f(r), g_rev(r). Aware evaluation computes (c0,c2);
/// unaware evaluation also computes p(1), independently of the incoming claim.
pub fn sumcheck_deg2_flipped_with_claim_mode<const CLAIM_AWARE: bool, F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    for &r in challenges {
        let half = f.len() / 2;
        let mut c0 = zero;
        let mut h1 = zero;
        let mut c2 = zero;
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            if !CLAIM_AWARE {
                h1 += (f0 + f1) * (g0 + g1);
            }
            c0 += f0 * g1;
            c2 += f1 * g0;
        }
        if CLAIM_AWARE {
            round_message((c0, c2));
        } else {
            round_message((c0, h1, c2));
        }
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            f[j] = f0 + r * f1;
            g[j] = g1 + r * g0;
        }
        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_flipped<F>(f: &mut Vec<F>, g: &mut Vec<F>, challenges: &[F], zero: F)
where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_flipped_with_claim_mode::<false, _>(f, g, challenges, zero);
}

pub fn sumcheck_deg2_flipped_claim_aware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_flipped_with_claim_mode::<true, _>(f, g, challenges, zero);
}

pub fn sumcheck_deg2_flipped_claim_unaware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_flipped_with_claim_mode::<false, _>(f, g, challenges, zero);
}

/// Absorb monomial weights into f during the first evaluation pass, then
/// continue with flipped binding in the same claim mode. Ends at f(a*r), g_rev(r).
pub fn sumcheck_deg2_eq_flipped_different_claims_with_claim_mode<const CLAIM_AWARE: bool, F>(
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
    let mut c0 = zero;
    let mut h1 = zero;
    let mut c2 = zero;
    for j in 0..half {
        let f0 = f[2 * j] * weights[2 * j];
        let f1 = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let g1 = g[2 * j + 1];
        if !CLAIM_AWARE {
            h1 += (f0 + f1) * (g0 + g1);
        }
        c0 += f0 * g1;
        c2 += f1 * g0;
        f[2 * j] = f0;
        f[2 * j + 1] = f1;
    }
    if CLAIM_AWARE {
        round_message((c0, c2));
    } else {
        round_message((c0, h1, c2));
    }
    let r = challenges[0];
    for j in 0..half {
        let f0 = f[2 * j];
        let f1 = f[2 * j + 1];
        let g0 = g[2 * j];
        let g1 = g[2 * j + 1];
        f[j] = f0 + r * f1;
        g[j] = g1 + r * g0;
    }
    f.truncate(half);
    g.truncate(half);
    sumcheck_deg2_flipped_with_claim_mode::<CLAIM_AWARE, _>(f, g, &challenges[1..], zero);
}

pub fn sumcheck_deg2_eq_flipped_different_claims<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_flipped_different_claims_with_claim_mode::<true, _>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_flipped_different_claims_claim_aware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_flipped_different_claims_with_claim_mode::<true, _>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_flipped_different_claims_claim_unaware<F>(
    f: &mut Vec<F>,
    g: &mut Vec<F>,
    suffix_eq: &[Vec<F>],
    challenges: &[F],
    zero: F,
) where
    F: Copy + Add<Output = F> + AddAssign + Mul<Output = F> + 'static,
{
    sumcheck_deg2_eq_flipped_different_claims_with_claim_mode::<false, _>(
        f, g, suffix_eq, challenges, zero,
    );
}
