use super::*;

/// `sumcheck_deg2_boolean` with the binding step written as one `mul_add`,
/// the operation the projective Fp128 kernels bind with.
pub fn sumcheck_deg2_boolean_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    challenges: &[Fp128],
    zero: Fp128,
) {
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

            f[j] = (f1 - f0).mul_add(r, f0);
            g[j] = (g1 - g0).mul_add(r, g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_projective_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    challenges: &[Fp128],
    zero: Fp128,
) {
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
            let sf = f0 + fi;
            let sg = g0 + gi;

            h0 += f0 * g0;
            h1 += sf * sg;
            h_inf += fi * gi;
        }

        round_message((h0, h1, h_inf));

        let r = challenges[round];

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            f[j] = fi.mul_add(r, f0);
            g[j] = gi.mul_add(r, g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_projective_1inf_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    challenges: &[Fp128],
    zero: Fp128,
) {
    let one = Fp128::one();
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h0 = zero;
        let mut h1 = zero;
        let mut h_inf = zero;

        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];
            let f0 = f1 - fi;
            let g0 = g1 - gi;

            h0 += f0 * g0;
            h1 += f1 * g1;
            h_inf += fi * gi;
        }

        round_message((h0, h1, h_inf));

        let r = challenges[round];
        let r_minus_one = r - one;

        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];

            f[j] = fi.mul_add(r_minus_one, f1);
            g[j] = gi.mul_add(r_minus_one, g1);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

/// `sumcheck_deg2_eq_gruen_boolean` with the binding step written as one
/// `mul_add`.
pub fn sumcheck_deg2_eq_gruen_boolean_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
    zero: Fp128,
) {
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

            f[j] = (f1 - f0).mul_add(r, f0);
            g[j] = (g1 - g0).mul_add(r, g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_eq_gruen_projective_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
    zero: Fp128,
) {
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
            let sf = wf0 + wfi;
            let sg = g0 + gi;

            q1 += sf * sg;
            q_inf += wfi * gi;
        }

        round_message((q1, q_inf));

        let r = challenges[round];

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            f[j] = fi.mul_add(r, f0);
            g[j] = gi.mul_add(r, g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

/// Gruen split-eq with the tables on {1,inf}.
///
/// The value at 1 and the leading coefficient of the round polynomial are
/// tied by the round claim, so the prover computes the value at 0 and the
/// leading coefficient. The value at 0 costs two subtractions per pair.
pub fn sumcheck_deg2_eq_gruen_projective_1inf_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
    zero: Fp128,
) {
    let one = Fp128::one();
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];

        let mut q0 = zero;
        let mut q_inf = zero;

        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];
            let ew = eq_rest[j];
            let wf1 = f1 * ew;
            let wfi = fi * ew;
            let f0 = wf1 - wfi;
            let g0 = g1 - gi;

            q0 += f0 * g0;
            q_inf += wfi * gi;
        }

        round_message((q0, q_inf));

        let r = challenges[round];
        let r_minus_one = r - one;

        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];

            f[j] = fi.mul_add(r_minus_one, f1);
            g[j] = gi.mul_add(r_minus_one, g1);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

struct Fp128Accum([u128; 4]);

impl Fp128Accum {
    #[inline(always)]
    fn zero() -> Self {
        Self([0u128; 4])
    }

    #[inline(always)]
    fn fmadd(&mut self, a: Fp128, b: Fp128) {
        let product = a.mul_wide(b);
        for i in 0..4 {
            self.0[i] += product[i] as u128;
        }
    }

    fn reduce(self) -> Fp128 {
        let mut limbs = [0u64; 5];
        let mut carry: u128 = 0;
        for i in 0..4 {
            let sum = self.0[i] + carry;
            limbs[i] = sum as u64;
            carry = sum >> 64;
        }
        limbs[4] = carry as u64;
        Fp128::solinas_reduce(&limbs)
    }
}

pub fn sumcheck_deg2_delayed_fp128(f: &mut Vec<Fp128>, g: &mut Vec<Fp128>, challenges: &[Fp128]) {
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h0 = Fp128Accum::zero();
        let mut h1 = Fp128Accum::zero();
        let mut h_inf = Fp128Accum::zero();

        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            let df = f1 - f0;
            let dg = g1 - g0;

            h0.fmadd(f0, g0);
            h1.fmadd(f1, g1);
            h_inf.fmadd(df, dg);
        }

        round_message((h0.reduce(), h1.reduce(), h_inf.reduce()));

        let r = challenges[round];
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            f[j] = (f1 - f0).mul_add(r, f0);
            g[j] = (g1 - g0).mul_add(r, g0);
        }
        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_eq_delayed_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
) {
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];

        let mut q1 = Fp128Accum::zero();
        let mut q_inf = Fp128Accum::zero();

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

            q1.fmadd(wf1, g1);
            q_inf.fmadd(df, dg);
        }

        round_message((q1.reduce(), q_inf.reduce()));

        let r = challenges[round];
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            f[j] = (f1 - f0).mul_add(r, f0);
            g[j] = (g1 - g0).mul_add(r, g0);
        }
        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_projective_delayed_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    challenges: &[Fp128],
) {
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h0 = Fp128Accum::zero();
        let mut h1 = Fp128Accum::zero();
        let mut h_inf = Fp128Accum::zero();

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            let sf = f0 + fi;
            let sg = g0 + gi;

            h0.fmadd(f0, g0);
            h1.fmadd(sf, sg);
            h_inf.fmadd(fi, gi);
        }

        round_message((h0.reduce(), h1.reduce(), h_inf.reduce()));

        let r = challenges[round];
        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            f[j] = fi.mul_add(r, f0);
            g[j] = gi.mul_add(r, g0);
        }
        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_projective_1inf_delayed_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    challenges: &[Fp128],
) {
    let one = Fp128::one();
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h0 = Fp128Accum::zero();
        let mut h1 = Fp128Accum::zero();
        let mut h_inf = Fp128Accum::zero();

        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];
            let f0 = f1 - fi;
            let g0 = g1 - gi;

            h0.fmadd(f0, g0);
            h1.fmadd(f1, g1);
            h_inf.fmadd(fi, gi);
        }

        round_message((h0.reduce(), h1.reduce(), h_inf.reduce()));

        let r = challenges[round];
        let r_minus_one = r - one;
        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];
            f[j] = fi.mul_add(r_minus_one, f1);
            g[j] = gi.mul_add(r_minus_one, g1);
        }
        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_eq_projective_delayed_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
) {
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];

        let mut q1 = Fp128Accum::zero();
        let mut q_inf = Fp128Accum::zero();

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            let ew = eq_rest[j];
            let wf0 = f0 * ew;
            let wfi = fi * ew;
            let sf = wf0 + wfi;
            let sg = g0 + gi;

            q1.fmadd(sf, sg);
            q_inf.fmadd(wfi, gi);
        }

        round_message((q1.reduce(), q_inf.reduce()));

        let r = challenges[round];
        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            f[j] = fi.mul_add(r, f0);
            g[j] = gi.mul_add(r, g0);
        }
        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_eq_projective_1inf_delayed_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
) {
    let one = Fp128::one();
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];

        let mut q0 = Fp128Accum::zero();
        let mut q_inf = Fp128Accum::zero();

        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];
            let ew = eq_rest[j];
            let wf1 = f1 * ew;
            let wfi = fi * ew;
            let f0 = wf1 - wfi;
            let g0 = g1 - gi;

            q0.fmadd(f0, g0);
            q_inf.fmadd(wfi, gi);
        }

        round_message((q0.reduce(), q_inf.reduce()));

        let r = challenges[round];
        let r_minus_one = r - one;
        for j in 0..half {
            let f1 = f[2 * j];
            let fi = f[2 * j + 1];
            let g1 = g[2 * j];
            let gi = g[2 * j + 1];
            f[j] = fi.mul_add(r_minus_one, f1);
            g[j] = gi.mul_add(r_minus_one, g1);
        }
        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_eq_projective_different_claims_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
    zero: Fp128,
) {
    if challenges.is_empty() {
        return;
    }
    let half = f.len() / 2;
    let weights = &suffix_eq[0];

    let mut h1 = zero;
    let mut h_inf = zero;

    for j in 0..half {
        let f0 = f[2 * j] * weights[2 * j];
        let fi = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];

        h1 += (f0 + fi) * (g0 + gi);
        h_inf += fi * gi;
        f[2 * j] = f0;
        f[2 * j + 1] = fi;
    }

    round_message((h1, h_inf));

    let r = challenges[0];

    for j in 0..half {
        let f0 = f[2 * j];
        let fi = f[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];

        f[j] = fi.mul_add(r, f0);
        g[j] = gi.mul_add(r, g0);
    }

    f.truncate(half);
    g.truncate(half);
    projective_different_claims_tail_fp128(f, g, &challenges[1..], zero);
}

pub fn sumcheck_deg2_eq_projective_different_claims_delayed_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    suffix_eq: &[Vec<Fp128>],
    challenges: &[Fp128],
) {
    if challenges.is_empty() {
        return;
    }
    let half = f.len() / 2;
    let weights = &suffix_eq[0];

    let mut h1 = Fp128Accum::zero();
    let mut h_inf = Fp128Accum::zero();

    for j in 0..half {
        let f0 = f[2 * j] * weights[2 * j];
        let fi = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];

        h1.fmadd(f0 + fi, g0 + gi);
        h_inf.fmadd(fi, gi);
        f[2 * j] = f0;
        f[2 * j + 1] = fi;
    }

    round_message((h1.reduce(), h_inf.reduce()));

    let r = challenges[0];
    for j in 0..half {
        let f0 = f[2 * j];
        let fi = f[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        f[j] = fi.mul_add(r, f0);
        g[j] = gi.mul_add(r, g0);
    }
    f.truncate(half);
    g.truncate(half);
    projective_different_claims_tail_delayed_fp128(f, g, &challenges[1..]);
}

pub(super) fn projective_different_claims_tail_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    challenges: &[Fp128],
    zero: Fp128,
) {
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h1 = zero;
        let mut h_inf = zero;

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            h1 += (f0 + fi) * (g0 + gi);
            h_inf += fi * gi;
        }

        round_message((h1, h_inf));

        let r = challenges[round];

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            f[j] = fi.mul_add(r, f0);
            g[j] = gi.mul_add(r, g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

pub(super) fn projective_different_claims_tail_delayed_fp128(
    f: &mut Vec<Fp128>,
    g: &mut Vec<Fp128>,
    challenges: &[Fp128],
) {
    for round in 0..challenges.len() {
        let half = f.len() / 2;

        let mut h1 = Fp128Accum::zero();
        let mut h_inf = Fp128Accum::zero();

        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];

            h1.fmadd(f0 + fi, g0 + gi);
            h_inf.fmadd(fi, gi);
        }

        round_message((h1.reduce(), h_inf.reduce()));

        let r = challenges[round];
        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            f[j] = fi.mul_add(r, f0);
            g[j] = gi.mul_add(r, g0);
        }
        f.truncate(half);
        g.truncate(half);
    }
}
