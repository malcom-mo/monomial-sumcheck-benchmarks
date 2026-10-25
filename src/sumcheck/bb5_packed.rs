use super::bb_ext::{BB5Accum, BB5MulByConst};
use super::*;

// The projective kernels accumulate the product of loaded values first, as
// the Boolean kernels do.  The loop is short of registers, and on the degree-5
// extensions LLVM's schedule for the other order is 5-10% slower.

#[cfg_attr(
    all(target_arch = "aarch64", target_feature = "neon"),
    allow(dead_code)
)]
pub fn bb5_eq_gruen_boolean_eval_ref_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5, BB5, BB5) {
    let half = f.len() / 2;
    let mut q1 = BB5::ZERO;
    let mut q0 = BB5::ZERO;
    let mut q_inf = BB5::ZERO;

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

    (q0, q1, q_inf)
}

pub fn bb5_eq_gruen_boolean_eval_ref(f: &[BB5], g: &[BB5], eq_rest: &[BB5]) -> (BB5, BB5) {
    let (_, v1, v2) = bb5_eq_gruen_boolean_eval_ref_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[cfg_attr(
    all(target_arch = "aarch64", target_feature = "neon"),
    allow(dead_code)
)]
pub fn bb5_eq_gruen_projective_eval_ref_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5, BB5, BB5) {
    let half = f.len() / 2;
    let mut q1 = BB5::ZERO;
    let mut q0 = BB5::ZERO;
    let mut q_inf = BB5::ZERO;

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

        if !CLAIM_AWARE {
            q0 += wf0 * g0;
        }
        q_inf += wfi * gi;
        q1 += sf * sg;
    }

    (q0, q1, q_inf)
}

pub fn bb5_eq_gruen_projective_eval_ref(f: &[BB5], g: &[BB5], eq_rest: &[BB5]) -> (BB5, BB5) {
    let (_, v1, v2) = bb5_eq_gruen_projective_eval_ref_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[inline(always)]
fn pack_bb5_chunk(values: [BB5; BB5_PACK_WIDTH]) -> PackedBB5 {
    PackedBB5::from_ext_slice(&values)
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[inline(always)]
fn sum_packed_bb5(acc: PackedBB5) -> BB5 {
    let mut sum = BB5::ZERO;
    for lane in 0..BB5_PACK_WIDTH {
        sum += <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&acc, lane);
    }
    sum
}

#[cfg_attr(
    all(target_arch = "aarch64", target_feature = "neon"),
    allow(dead_code)
)]
pub fn bb5_eq_delayed_eval_ref_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    let half = f.len() / 2;
    let mut q1 = BB5Accum::zero();
    let mut q0 = BB5Accum::zero();
    let mut q_inf = BB5Accum::zero();

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
            q0.fmadd(wf0, g0);
        }
        q1.fmadd(wf1, g1);
        q_inf.fmadd(df, dg);
    }

    (q0, q1, q_inf)
}

pub fn bb5_eq_delayed_eval_ref(f: &[BB5], g: &[BB5], eq_rest: &[BB5]) -> (BB5Accum, BB5Accum) {
    let (_, v1, v2) = bb5_eq_delayed_eval_ref_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[cfg_attr(
    all(target_arch = "aarch64", target_feature = "neon"),
    allow(dead_code)
)]
pub fn bb5_eq_projective_delayed_eval_ref_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    let half = f.len() / 2;
    let mut q1 = BB5Accum::zero();
    let mut q0 = BB5Accum::zero();
    let mut q_inf = BB5Accum::zero();

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

        if !CLAIM_AWARE {
            q0.fmadd(wf0, g0);
        }
        q_inf.fmadd(wfi, gi);
        q1.fmadd(sf, sg);
    }

    (q0, q1, q_inf)
}

pub fn bb5_eq_projective_delayed_eval_ref(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum) {
    let (_, v1, v2) = bb5_eq_projective_delayed_eval_ref_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[inline(always)]
unsafe fn bb_neon_sum_mul_4(a: PackedBabyBearNeon, b: PackedBabyBearNeon) -> u128 {
    let a_vec = vld1q_u32(a.as_slice().as_ptr() as *const u32);
    let b_vec = vld1q_u32(b.as_slice().as_ptr() as *const u32);
    let lo = vmull_u32(vget_low_u32(a_vec), vget_low_u32(b_vec));
    let hi = vmull_high_u32(a_vec, b_vec);
    vaddvq_u64(vaddq_u64(lo, hi)) as u128
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[inline(always)]
fn bb5_accum_fmadd_packed(acc: &mut BB5Accum, a: PackedBB5, b: PackedBB5) {
    let a = a.as_basis_coefficients_slice();
    let b = b.as_basis_coefficients_slice();
    let a0 = a[0];
    let a1 = a[1];
    let a2 = a[2];
    let a3 = a[3];
    let a4 = a[4];
    let b0 = b[0];
    let b1 = b[1];
    let b2 = b[2];
    let b3 = b[3];
    let b4 = b[4];

    unsafe {
        acc.c[0] += bb_neon_sum_mul_4(a0, b0);
        acc.c[1] += bb_neon_sum_mul_4(a0, b1) + bb_neon_sum_mul_4(a1, b0);
        acc.c[2] +=
            bb_neon_sum_mul_4(a0, b2) + bb_neon_sum_mul_4(a1, b1) + bb_neon_sum_mul_4(a2, b0);
        acc.c[3] += bb_neon_sum_mul_4(a0, b3)
            + bb_neon_sum_mul_4(a1, b2)
            + bb_neon_sum_mul_4(a2, b1)
            + bb_neon_sum_mul_4(a3, b0);
        acc.c[4] += bb_neon_sum_mul_4(a0, b4)
            + bb_neon_sum_mul_4(a1, b3)
            + bb_neon_sum_mul_4(a2, b2)
            + bb_neon_sum_mul_4(a3, b1)
            + bb_neon_sum_mul_4(a4, b0);
        acc.c[5] += bb_neon_sum_mul_4(a1, b4)
            + bb_neon_sum_mul_4(a2, b3)
            + bb_neon_sum_mul_4(a3, b2)
            + bb_neon_sum_mul_4(a4, b1);
        acc.c[6] +=
            bb_neon_sum_mul_4(a2, b4) + bb_neon_sum_mul_4(a3, b3) + bb_neon_sum_mul_4(a4, b2);
        acc.c[7] += bb_neon_sum_mul_4(a3, b4) + bb_neon_sum_mul_4(a4, b3);
        acc.c[8] += bb_neon_sum_mul_4(a4, b4);
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn bb5_eq_gruen_boolean_eval_packed_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5, BB5, BB5) {
    let half = f.len() / 2;
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    let mut q1_packed = PackedBB5::ZERO;
    let mut q0_packed = PackedBB5::ZERO;
    let mut q_inf_packed = PackedBB5::ZERO;
    let mut q1_tail = BB5::ZERO;
    let mut q0_tail = BB5::ZERO;
    let mut q_inf_tail = BB5::ZERO;

    for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
        let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
        let f1 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
        let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
        let g1 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
        let ew = PackedBB5::from_ext_slice(&eq_rest[j..j + BB5_PACK_WIDTH]);
        let wf0 = f0 * ew;
        let wf1 = f1 * ew;
        let df = wf1 - wf0;
        let dg = g1 - g0;

        if !CLAIM_AWARE {
            q0_packed += wf0 * g0;
        }
        q1_packed += wf1 * g1;
        q_inf_packed += df * dg;
    }

    for j in packed_end..half {
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
            q0_tail += wf0 * g0;
        }
        q1_tail += wf1 * g1;
        q_inf_tail += df * dg;
    }

    (
        sum_packed_bb5(q0_packed) + q0_tail,
        sum_packed_bb5(q1_packed) + q1_tail,
        sum_packed_bb5(q_inf_packed) + q_inf_tail,
    )
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn bb5_eq_delayed_eval_packed_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    let half = f.len() / 2;
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    let mut q1 = BB5Accum::zero();
    let mut q0 = BB5Accum::zero();
    let mut q_inf = BB5Accum::zero();

    for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
        let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
        let f1 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
        let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
        let g1 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
        let ew = PackedBB5::from_ext_slice(&eq_rest[j..j + BB5_PACK_WIDTH]);
        let wf0 = f0 * ew;
        let wf1 = f1 * ew;
        let df = wf1 - wf0;
        let dg = g1 - g0;

        if !CLAIM_AWARE {
            bb5_accum_fmadd_packed(&mut q0, wf0, g0);
        }
        bb5_accum_fmadd_packed(&mut q1, wf1, g1);
        bb5_accum_fmadd_packed(&mut q_inf, df, dg);
    }

    for j in packed_end..half {
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
            q0.fmadd(wf0, g0);
        }
        q1.fmadd(wf1, g1);
        q_inf.fmadd(df, dg);
    }

    (q0, q1, q_inf)
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub fn bb5_eq_gruen_projective_eval_packed_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5, BB5, BB5) {
    let half = f.len() / 2;
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    let mut q1_packed = PackedBB5::ZERO;
    let mut q0_packed = PackedBB5::ZERO;
    let mut q_inf_packed = PackedBB5::ZERO;
    let mut q1_tail = BB5::ZERO;
    let mut q0_tail = BB5::ZERO;
    let mut q_inf_tail = BB5::ZERO;

    for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
        let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
        let fi = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
        let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
        let gi = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
        let ew = PackedBB5::from_ext_slice(&eq_rest[j..j + BB5_PACK_WIDTH]);
        let wf0 = f0 * ew;
        let wfi = fi * ew;
        let sf = wf0 + wfi;
        let sg = g0 + gi;

        if !CLAIM_AWARE {
            q0_packed += wf0 * g0;
        }
        q_inf_packed += wfi * gi;
        q1_packed += sf * sg;
    }

    for j in packed_end..half {
        let f0 = f[2 * j];
        let fi = f[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        let ew = eq_rest[j];
        let wf0 = f0 * ew;
        let wfi = fi * ew;
        let sf = wf0 + wfi;
        let sg = g0 + gi;

        if !CLAIM_AWARE {
            q0_tail += wf0 * g0;
        }
        q_inf_tail += wfi * gi;
        q1_tail += sf * sg;
    }

    (
        sum_packed_bb5(q0_packed) + q0_tail,
        sum_packed_bb5(q1_packed) + q1_tail,
        sum_packed_bb5(q_inf_packed) + q_inf_tail,
    )
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub fn bb5_eq_gruen_projective_eval_packed(f: &[BB5], g: &[BB5], eq_rest: &[BB5]) -> (BB5, BB5) {
    let (_, v1, v2) = bb5_eq_gruen_projective_eval_packed_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn bb5_eq_projective_delayed_eval_packed_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    let half = f.len() / 2;
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    let mut q1 = BB5Accum::zero();
    let mut q0 = BB5Accum::zero();
    let mut q_inf = BB5Accum::zero();

    for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
        let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
        let fi = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
        let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
        let gi = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
        let ew = PackedBB5::from_ext_slice(&eq_rest[j..j + BB5_PACK_WIDTH]);
        let wf0 = f0 * ew;
        let wfi = fi * ew;
        let sf = wf0 + wfi;
        let sg = g0 + gi;

        if !CLAIM_AWARE {
            bb5_accum_fmadd_packed(&mut q0, wf0, g0);
        }
        bb5_accum_fmadd_packed(&mut q_inf, wfi, gi);
        bb5_accum_fmadd_packed(&mut q1, sf, sg);
    }

    for j in packed_end..half {
        let f0 = f[2 * j];
        let fi = f[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        let ew = eq_rest[j];
        let wf0 = f0 * ew;
        let wfi = fi * ew;
        let sf = wf0 + wfi;
        let sg = g0 + gi;

        if !CLAIM_AWARE {
            q0.fmadd(wf0, g0);
        }
        q_inf.fmadd(wfi, gi);
        q1.fmadd(sf, sg);
    }

    (q0, q1, q_inf)
}

#[inline(always)]
pub fn bb5_eq_gruen_boolean_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5, BB5, BB5) {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        bb5_eq_gruen_boolean_eval_packed_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    {
        bb5_eq_gruen_boolean_eval_ref_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
}

pub fn bb5_eq_gruen_boolean_eval(f: &[BB5], g: &[BB5], eq_rest: &[BB5]) -> (BB5, BB5) {
    let (_, v1, v2) = bb5_eq_gruen_boolean_eval_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[inline(always)]
pub fn bb5_eq_gruen_projective_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5, BB5, BB5) {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        bb5_eq_gruen_projective_eval_packed_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    {
        bb5_eq_gruen_projective_eval_ref_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
}

pub fn bb5_eq_gruen_projective_eval(f: &[BB5], g: &[BB5], eq_rest: &[BB5]) -> (BB5, BB5) {
    let (_, v1, v2) = bb5_eq_gruen_projective_eval_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[inline(always)]
pub fn bb5_eq_delayed_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        bb5_eq_delayed_eval_packed_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    {
        bb5_eq_delayed_eval_ref_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
}

pub fn bb5_eq_delayed_eval(f: &[BB5], g: &[BB5], eq_rest: &[BB5]) -> (BB5Accum, BB5Accum) {
    let (_, v1, v2) = bb5_eq_delayed_eval_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

#[inline(always)]
pub fn bb5_eq_projective_delayed_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        bb5_eq_projective_delayed_eval_packed_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    {
        bb5_eq_projective_delayed_eval_ref_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest)
    }
}

pub fn bb5_eq_projective_delayed_eval(
    f: &[BB5],
    g: &[BB5],
    eq_rest: &[BB5],
) -> (BB5Accum, BB5Accum) {
    let (_, v1, v2) = bb5_eq_projective_delayed_eval_with_claim_mode::<true>(f, g, eq_rest);
    (v1, v2)
}

pub fn sumcheck_deg2_eq_gruen_boolean_bb5_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];
        let (q0, q1, q_inf) =
            bb5_eq_gruen_boolean_eval_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest);
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

    let _ = black_box(zero);
}

pub fn sumcheck_deg2_eq_gruen_boolean_bb5(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_gruen_boolean_bb5_with_claim_mode::<true>(f, g, suffix_eq, challenges, zero);
}

pub fn sumcheck_deg2_eq_gruen_boolean_bb5_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_gruen_boolean_bb5_with_claim_mode::<true>(f, g, suffix_eq, challenges, zero);
}

pub fn sumcheck_deg2_eq_gruen_boolean_bb5_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_gruen_boolean_bb5_with_claim_mode::<false>(f, g, suffix_eq, challenges, zero);
}

pub fn sumcheck_deg2_eq_delayed_bb5_packed_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];
        let (q0, q1, q_inf) = bb5_eq_delayed_eval_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest);
        if CLAIM_AWARE {
            round_message((q1.reduce(), q_inf.reduce()));
        } else {
            round_message((q0.reduce(), q1.reduce(), q_inf.reduce()));
        }

        let r = challenges[round];
        let mul_r = BB5MulByConst::new(r);
        for j in 0..half {
            let f0 = f[2 * j];
            let f1 = f[2 * j + 1];
            let g0 = g[2 * j];
            let g1 = g[2 * j + 1];
            f[j] = f0 + mul_r.apply(f1 - f0);
            g[j] = g0 + mul_r.apply(g1 - g0);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_eq_delayed_bb5_packed(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_delayed_bb5_packed_with_claim_mode::<true>(f, g, suffix_eq, challenges);
}

pub fn sumcheck_deg2_eq_delayed_bb5_packed_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_delayed_bb5_packed_with_claim_mode::<true>(f, g, suffix_eq, challenges);
}

pub fn sumcheck_deg2_eq_delayed_bb5_packed_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_delayed_bb5_packed_with_claim_mode::<false>(f, g, suffix_eq, challenges);
}

pub fn sumcheck_deg2_eq_gruen_projective_bb5_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];
        let (q0, q1, q_inf) =
            bb5_eq_gruen_projective_eval_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest);
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

    let _ = black_box(zero);
}

pub fn sumcheck_deg2_eq_gruen_projective_bb5(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_gruen_projective_bb5_with_claim_mode::<true>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_gruen_projective_bb5_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_gruen_projective_bb5_with_claim_mode::<true>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_gruen_projective_bb5_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_gruen_projective_bb5_with_claim_mode::<false>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_projective_delayed_bb5_packed_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    let n = challenges.len();
    for round in 0..n {
        let half = f.len() / 2;
        let eq_rest = &suffix_eq[round + 1];
        let (q0, q1, q_inf) =
            bb5_eq_projective_delayed_eval_with_claim_mode::<CLAIM_AWARE>(f, g, eq_rest);
        if CLAIM_AWARE {
            round_message((q1.reduce(), q_inf.reduce()));
        } else {
            round_message((q0.reduce(), q1.reduce(), q_inf.reduce()));
        }

        let r = challenges[round];
        let mul_r = BB5MulByConst::new(r);
        for j in 0..half {
            let f0 = f[2 * j];
            let fi = f[2 * j + 1];
            let g0 = g[2 * j];
            let gi = g[2 * j + 1];
            f[j] = f0 + mul_r.apply(fi);
            g[j] = g0 + mul_r.apply(gi);
        }

        f.truncate(half);
        g.truncate(half);
    }
}

pub fn sumcheck_deg2_eq_projective_delayed_bb5_packed(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_projective_delayed_bb5_packed_with_claim_mode::<true>(
        f, g, suffix_eq, challenges,
    );
}

pub fn sumcheck_deg2_eq_projective_delayed_bb5_packed_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_projective_delayed_bb5_packed_with_claim_mode::<true>(
        f, g, suffix_eq, challenges,
    );
}

pub fn sumcheck_deg2_eq_projective_delayed_bb5_packed_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_projective_delayed_bb5_packed_with_claim_mode::<false>(
        f, g, suffix_eq, challenges,
    );
}

fn bb5_projective_preweighted_stored_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut [BB5],
    g: &[BB5],
    weights: &[BB5],
) -> (BB5, BB5, BB5) {
    let half = f.len() / 2;
    let mut h0 = BB5::ZERO;
    let mut h1 = BB5::ZERO;
    let mut h_inf = BB5::ZERO;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    let packed_end = 0;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        let mut h0_packed = PackedBB5::ZERO;
        let mut h1_packed = PackedBB5::ZERO;
        let mut h_inf_packed = PackedBB5::ZERO;
        for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
            let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
            let fi = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
            let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
            let gi = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
            let wf0 = f0 * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane)]));
            let wfi = fi * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane) + 1]));
            h_inf_packed += wfi * gi;
            if !CLAIM_AWARE {
                h0_packed += wf0 * g0;
            }
            h1_packed += (wf0 + wfi) * (g0 + gi);
            for lane in 0..BB5_PACK_WIDTH {
                f[2 * (j + lane)] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wf0, lane);
                f[2 * (j + lane) + 1] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wfi, lane);
            }
        }
        if !CLAIM_AWARE {
            h0 += sum_packed_bb5(h0_packed);
        }
        h1 += sum_packed_bb5(h1_packed);
        h_inf += sum_packed_bb5(h_inf_packed);
    }
    for j in packed_end..half {
        let wf0 = f[2 * j] * weights[2 * j];
        let wfi = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        h_inf += wfi * gi;
        if !CLAIM_AWARE {
            h0 += wf0 * g0;
        }
        h1 += (wf0 + wfi) * (g0 + gi);
        f[2 * j] = wf0;
        f[2 * j + 1] = wfi;
    }
    (h0, h1, h_inf)
}

pub fn sumcheck_deg2_eq_projective_different_claims_bb5_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    if challenges.is_empty() {
        return;
    }
    let half = f.len() / 2;

    let (h0, h1, h_inf) =
        bb5_projective_preweighted_stored_eval_with_claim_mode::<CLAIM_AWARE>(f, g, &suffix_eq[0]);
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

pub fn sumcheck_deg2_eq_projective_different_claims_bb5(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_projective_different_claims_bb5_with_claim_mode::<true>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims_bb5_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_projective_different_claims_bb5_with_claim_mode::<true>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims_bb5_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_projective_different_claims_bb5_with_claim_mode::<false>(
        f, g, suffix_eq, challenges, zero,
    );
}

fn bb5_projective_preweighted_stored_delayed_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut [BB5],
    g: &[BB5],
    weights: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    let half = f.len() / 2;
    let mut h0 = BB5Accum::zero();
    let mut h1 = BB5Accum::zero();
    let mut h_inf = BB5Accum::zero();
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    let packed_end = 0;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
            let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
            let fi = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
            let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
            let gi = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
            let wf0 = f0 * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane)]));
            let wfi = fi * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane) + 1]));
            bb5_accum_fmadd_packed(&mut h_inf, wfi, gi);
            if !CLAIM_AWARE {
                bb5_accum_fmadd_packed(&mut h0, wf0, g0);
            }
            bb5_accum_fmadd_packed(&mut h1, wf0 + wfi, g0 + gi);
            for lane in 0..BB5_PACK_WIDTH {
                f[2 * (j + lane)] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wf0, lane);
                f[2 * (j + lane) + 1] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wfi, lane);
            }
        }
    }
    for j in packed_end..half {
        let wf0 = f[2 * j] * weights[2 * j];
        let wfi = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        h_inf.fmadd(wfi, gi);
        if !CLAIM_AWARE {
            h0.fmadd(wf0, g0);
        }
        h1.fmadd(wf0 + wfi, g0 + gi);
        f[2 * j] = wf0;
        f[2 * j + 1] = wfi;
    }
    (h0, h1, h_inf)
}

pub fn sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed_with_claim_mode<
    const CLAIM_AWARE: bool,
>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    if challenges.is_empty() {
        return;
    }
    let half = f.len() / 2;

    let (h0, h1, h_inf) = bb5_projective_preweighted_stored_delayed_eval_with_claim_mode::<
        CLAIM_AWARE,
    >(f, g, &suffix_eq[0]);
    if CLAIM_AWARE {
        round_message((h1.reduce(), h_inf.reduce()));
    } else {
        round_message((h0.reduce(), h1.reduce(), h_inf.reduce()));
    }
    let r = challenges[0];
    let mul_r = BB5MulByConst::new(r);
    for j in 0..half {
        let f0 = f[2 * j];
        let fi = f[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        f[j] = f0 + mul_r.apply(fi);
        g[j] = g0 + mul_r.apply(gi);
    }
    f.truncate(half);
    g.truncate(half);
    projective_different_claims_tail_delayed_bb5_with_claim_mode::<CLAIM_AWARE>(
        f,
        g,
        &challenges[1..],
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed_with_claim_mode::<true>(
        f, g, suffix_eq, challenges,
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed_with_claim_mode::<true>(
        f, g, suffix_eq, challenges,
    );
}

pub fn sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed_with_claim_mode::<false>(
        f, g, suffix_eq, challenges,
    );
}

fn bb5_preweighted_stored_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut [BB5],
    g: &[BB5],
    weights: &[BB5],
) -> (BB5, BB5, BB5) {
    let half = f.len() / 2;
    let mut c0 = BB5::ZERO;
    let mut c2 = BB5::ZERO;
    let mut h1 = BB5::ZERO;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    let packed_end = 0;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        let mut c0_packed = PackedBB5::ZERO;
        let mut c2_packed = PackedBB5::ZERO;
        let mut h1_packed = PackedBB5::ZERO;
        for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
            let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
            let fi = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
            let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
            let gi = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
            let wf0 = f0 * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane)]));
            let wfi = fi * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane) + 1]));
            c2_packed += wfi * g0;
            if !CLAIM_AWARE {
                h1_packed += (wf0 + wfi) * (g0 + gi);
            }
            c0_packed += wf0 * gi;
            for lane in 0..BB5_PACK_WIDTH {
                f[2 * (j + lane)] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wf0, lane);
                f[2 * (j + lane) + 1] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wfi, lane);
            }
        }
        c0 += sum_packed_bb5(c0_packed);
        c2 += sum_packed_bb5(c2_packed);
        if !CLAIM_AWARE {
            h1 += sum_packed_bb5(h1_packed);
        }
    }
    for j in packed_end..half {
        let wf0 = f[2 * j] * weights[2 * j];
        let wfi = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        c2 += wfi * g0;
        if !CLAIM_AWARE {
            h1 += (wf0 + wfi) * (g0 + gi);
        }
        c0 += wf0 * gi;
        f[2 * j] = wf0;
        f[2 * j + 1] = wfi;
    }
    (c0, h1, c2)
}

pub fn sumcheck_deg2_eq_flipped_different_claims_bb5_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    if challenges.is_empty() {
        return;
    }
    let half = f.len() / 2;

    let (c0, h1, c2) =
        bb5_preweighted_stored_eval_with_claim_mode::<CLAIM_AWARE>(f, g, &suffix_eq[0]);
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

pub fn sumcheck_deg2_eq_flipped_different_claims_bb5(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_flipped_different_claims_bb5_with_claim_mode::<true>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_flipped_different_claims_bb5_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_flipped_different_claims_bb5_with_claim_mode::<true>(
        f, g, suffix_eq, challenges, zero,
    );
}

pub fn sumcheck_deg2_eq_flipped_different_claims_bb5_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
    zero: BB5,
) {
    sumcheck_deg2_eq_flipped_different_claims_bb5_with_claim_mode::<false>(
        f, g, suffix_eq, challenges, zero,
    );
}

fn bb5_preweighted_stored_delayed_eval_with_claim_mode<const CLAIM_AWARE: bool>(
    f: &mut [BB5],
    g: &[BB5],
    weights: &[BB5],
) -> (BB5Accum, BB5Accum, BB5Accum) {
    let half = f.len() / 2;
    let mut c0 = BB5Accum::zero();
    let mut c2 = BB5Accum::zero();
    let mut h1 = BB5Accum::zero();
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    let packed_end = half / BB5_PACK_WIDTH * BB5_PACK_WIDTH;
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    let packed_end = 0;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        for j in (0..packed_end).step_by(BB5_PACK_WIDTH) {
            let f0 = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane)]));
            let fi = pack_bb5_chunk(core::array::from_fn(|lane| f[2 * (j + lane) + 1]));
            let g0 = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane)]));
            let gi = pack_bb5_chunk(core::array::from_fn(|lane| g[2 * (j + lane) + 1]));
            let wf0 = f0 * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane)]));
            let wfi = fi * pack_bb5_chunk(core::array::from_fn(|lane| weights[2 * (j + lane) + 1]));
            bb5_accum_fmadd_packed(&mut c2, wfi, g0);
            if !CLAIM_AWARE {
                bb5_accum_fmadd_packed(&mut h1, wf0 + wfi, g0 + gi);
            }
            bb5_accum_fmadd_packed(&mut c0, wf0, gi);
            for lane in 0..BB5_PACK_WIDTH {
                f[2 * (j + lane)] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wf0, lane);
                f[2 * (j + lane) + 1] =
                    <PackedBB5 as PackedFieldExtension<BabyBear, BB5>>::extract(&wfi, lane);
            }
        }
    }
    for j in packed_end..half {
        let wf0 = f[2 * j] * weights[2 * j];
        let wfi = f[2 * j + 1] * weights[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        c2.fmadd(wfi, g0);
        if !CLAIM_AWARE {
            h1.fmadd(wf0 + wfi, g0 + gi);
        }
        c0.fmadd(wf0, gi);
        f[2 * j] = wf0;
        f[2 * j + 1] = wfi;
    }
    (c0, h1, c2)
}

pub fn sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed_with_claim_mode<
    const CLAIM_AWARE: bool,
>(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    if challenges.is_empty() {
        return;
    }
    let half = f.len() / 2;

    let (c0, h1, c2) =
        bb5_preweighted_stored_delayed_eval_with_claim_mode::<CLAIM_AWARE>(f, g, &suffix_eq[0]);
    if CLAIM_AWARE {
        round_message((c0.reduce(), c2.reduce()));
    } else {
        round_message((c0.reduce(), h1.reduce(), c2.reduce()));
    }
    let r = challenges[0];
    let mul_r = BB5MulByConst::new(r);
    for j in 0..half {
        let f0 = f[2 * j];
        let fi = f[2 * j + 1];
        let g0 = g[2 * j];
        let gi = g[2 * j + 1];
        f[j] = f0 + mul_r.apply(fi);
        g[j] = gi + mul_r.apply(g0);
    }
    f.truncate(half);
    g.truncate(half);
    sumcheck_deg2_flipped_delayed_bb5_with_claim_mode::<CLAIM_AWARE>(f, g, &challenges[1..]);
}

pub fn sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed_with_claim_mode::<true>(
        f, g, suffix_eq, challenges,
    );
}

pub fn sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed_claim_aware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed_with_claim_mode::<true>(
        f, g, suffix_eq, challenges,
    );
}

pub fn sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed_claim_unaware(
    f: &mut Vec<BB5>,
    g: &mut Vec<BB5>,
    suffix_eq: &[Vec<BB5>],
    challenges: &[BB5],
) {
    sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed_with_claim_mode::<false>(
        f, g, suffix_eq, challenges,
    );
}
