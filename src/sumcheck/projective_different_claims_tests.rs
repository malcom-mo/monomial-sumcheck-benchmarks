//! Check the preweighted protocol against direct contraction of original inputs.
use super::*;
use binius_field::Field as _;
use jolt_field::Zero as _;

fn run<F: Copy, T: Copy + 'static>(
    f: &[F],
    g: &[F],
    kernel: impl FnOnce(&mut Vec<F>, &mut Vec<F>),
) -> (Vec<T>, Vec<F>, Vec<F>) {
    let (mut f, mut g) = (f.to_vec(), g.to_vec());
    let messages = round_messages_of(|| kernel(&mut f, &mut g));
    (messages, f, g)
}

fn restrict<F>(table: &[F], point: &[F], zero: F) -> Vec<F>
where
    F: Copy + AddAssign + Mul<Output = F>,
{
    table
        .chunks_exact(1 << point.len())
        .map(|chunk| {
            let mut sum = zero;
            for (i, &value) in chunk.iter().enumerate() {
                let mut term = value;
                for (k, &r) in point.iter().enumerate() {
                    if i >> k & 1 != 0 {
                        term = term * r;
                    }
                }
                sum += term;
            }
            sum
        })
        .collect()
}

fn check_protocol<F>(f: &[F], g: &[F], a: &[F], ch: &[F], zero: F, one: F)
where
    F: Copy
        + Add<Output = F>
        + AddAssign
        + Sub<Output = F>
        + Mul<Output = F>
        + PartialEq
        + std::fmt::Debug
        + 'static,
{
    let weighted: Vec<_> = f
        .iter()
        .enumerate()
        .map(|(i, &f)| {
            a.iter()
                .enumerate()
                .fold(f, |v, (k, &a)| if i >> k & 1 != 0 { v * a } else { v })
        })
        .collect();
    let eq = build_suffix_eq_tables_projective(a, one);
    let (messages, ff, gg) = run::<_, (F, F)>(f, g, |f, g| {
        sumcheck_deg2_eq_projective_different_claims(f, g, &eq, ch, zero)
    });
    let dot = |f: &[F], g: &[F]| f.iter().zip(g).fold(zero, |s, (&f, &g)| s + f * g);
    let mut claim = dot(&weighted, g);
    assert_eq!(messages.len(), ch.len());
    for (k, &(h1, h_inf)) in messages.iter().enumerate() {
        let fr = restrict(&weighted, &ch[..k], zero);
        let gr = restrict(g, &ch[..k], zero);
        let (mut e1, mut ei) = (zero, zero);
        for (fp, gp) in fr.chunks_exact(2).zip(gr.chunks_exact(2)) {
            e1 += (fp[0] + fp[1]) * (gp[0] + gp[1]);
            ei += fp[1] * gp[1];
        }
        assert_eq!((h1, h_inf), (e1, ei));
        let h0 = claim - h_inf;
        let r = ch[k];
        claim = h0 + r * (h1 - h0 - h_inf) + r * r * h_inf;
        assert_eq!(
            claim,
            dot(
                &restrict(&weighted, &ch[..=k], zero),
                &restrict(g, &ch[..=k], zero)
            )
        );
    }
    let ar: Vec<_> = a.iter().zip(ch).map(|(&a, &r)| a * r).collect();
    assert_eq!(ff, restrict(f, &ar, zero));
    assert_eq!(gg, restrict(g, ch, zero));
    assert_eq!(claim, ff[0] * gg[0]);
    // The original three-value unweighted loop provides an additional check
    // after preweighting outside the tested kernel.
    let expected = run::<_, (F, F, F)>(&weighted, g, |f, g| {
        sumcheck_deg2_projective(f, g, ch, zero)
    });
    assert_eq!(
        messages,
        expected
            .0
            .iter()
            .map(|&(_, h1, hi)| (h1, hi))
            .collect::<Vec<_>>()
    );
    assert_eq!((ff, gg), (expected.1, expected.2));
}

macro_rules! field_tests {
    ($name:ident,$field:ty,$make:ident,$zero:expr,$one:expr,$eager:ident,$delayed:ident) => {
        #[test]
        fn $name() {
            let (zero, one) = ($zero, $one);
            for n in [0, 1, 2, 3, 6] {
                let (f, g) = ($make(1 << n), $make(1 << n));
                let mixed: Vec<_> = (0..n)
                    .map(|i| if i % 2 == 0 { zero } else { one })
                    .collect();
                for ch in [$make(n), vec![zero; n], vec![one; n]] {
                    for a in [$make(n), vec![zero; n], vec![one; n], mixed.clone()] {
                        check_protocol(&f, &g, &a, &ch, zero, one);
                        let eq = build_suffix_eq_tables_projective(&a, one);
                        let expected = run::<$field, ($field, $field)>(&f, &g, |f, g| {
                            sumcheck_deg2_eq_projective_different_claims(f, g, &eq, &ch, zero)
                        });
                        assert_eq!(run(&f, &g, |f, g| $eager(f, g, &eq, &ch, zero)), expected);
                        assert_eq!(run(&f, &g, |f, g| $delayed(f, g, &eq, &ch)), expected);
                    }
                }
            }
        }
    };
}
field_tests!(
    bn254_preweighted,
    BN254Fr,
    make_bn254,
    BN254Fr::ZERO,
    BN254Fr::from(1),
    sumcheck_deg2_eq_projective_different_claims,
    sumcheck_deg2_eq_projective_different_claims_delayed_bn254
);
field_tests!(
    fp128_preweighted,
    Fp128,
    make_fp128,
    Fp128::zero(),
    Fp128::one(),
    sumcheck_deg2_eq_projective_different_claims_fp128,
    sumcheck_deg2_eq_projective_different_claims_delayed_fp128
);
field_tests!(
    bb4_preweighted,
    BB4,
    make_bb4,
    BB4::ZERO,
    BB4::ONE,
    sumcheck_deg2_eq_projective_different_claims,
    sumcheck_deg2_eq_projective_different_claims_delayed_bb4
);
field_tests!(
    bb5_preweighted,
    BB5,
    make_bb5,
    BB5::ZERO,
    BB5::ONE,
    sumcheck_deg2_eq_projective_different_claims_bb5,
    sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed
);
field_tests!(
    bb5_scalar_preweighted,
    BB5,
    make_bb5,
    BB5::ZERO,
    BB5::ONE,
    sumcheck_deg2_eq_projective_different_claims,
    sumcheck_deg2_eq_projective_different_claims_delayed_bb5
);
field_tests!(
    kb5_preweighted,
    KB5,
    make_kb5,
    KB5::ZERO,
    KB5::ONE,
    sumcheck_deg2_eq_projective_different_claims,
    sumcheck_deg2_eq_projective_different_claims_delayed_kb5
);
field_tests!(
    gf128_preweighted,
    GF128,
    make_gf128,
    GF128::ZERO,
    GF128::ONE,
    sumcheck_deg2_eq_projective_different_claims,
    sumcheck_deg2_eq_projective_different_claims_delayed_gf128
);

#[test]
fn bn254_branch_free_preweighted() {
    for n in [0, 1, 3, 6] {
        let (f, g, ch, a) = (
            make_bn254(1 << n),
            make_bn254(1 << n),
            make_bn254(n),
            make_bn254(n),
        );
        let wrap = |xs: &[BN254Fr]| xs.iter().copied().map(Bn254BranchFree).collect::<Vec<_>>();
        let (fb, gb, chb, ab) = (wrap(&f), wrap(&g), wrap(&ch), wrap(&a));
        check_protocol(
            &fb,
            &gb,
            &ab,
            &chb,
            Bn254BranchFree::ZERO,
            Bn254BranchFree::ONE,
        );
        let eq = build_suffix_eq_tables_projective(&a, BN254Fr::from(1));
        let eqb = build_suffix_eq_tables_projective(&ab, Bn254BranchFree::ONE);
        let expected = run::<_, (BN254Fr, BN254Fr)>(&f, &g, |f, g| {
            sumcheck_deg2_eq_projective_different_claims(f, g, &eq, &ch, BN254Fr::ZERO)
        });
        let got = run::<_, (BN254Fr, BN254Fr)>(&fb, &gb, |f, g| {
            sumcheck_deg2_eq_projective_different_claims_delayed_bn254_generic(f, g, &eqb, &chb)
        });
        assert_eq!(
            got,
            (expected.0.clone(), wrap(&expected.1), wrap(&expected.2))
        );
        assert_eq!(
            run(&f, &g, |f, g| {
                sumcheck_deg2_eq_projective_different_claims_delayed_bn254_generic(f, g, &eq, &ch)
            }),
            expected
        );
    }
}

#[test]
fn bn254_upper_preweighted() {
    for n in [0, 1, 3, 6] {
        let (f, g, a) = (make_bn254(1 << n), make_bn254(1 << n), make_bn254(n));
        let eq = build_suffix_eq_tables_projective(&a, BN254Fr::from(1));
        let (mut ch, mut limbs) = make_bn254_upper_limb_challenges(n);
        if n > 0 {
            ch[0] = BN254Fr::ZERO;
            limbs[0] = (0, 0);
        }
        let expected = run::<_, (BN254Fr, BN254Fr)>(&f, &g, |f, g| {
            sumcheck_deg2_eq_projective_different_claims(f, g, &eq, &ch, BN254Fr::ZERO)
        });
        assert_eq!(
            run(&f, &g, |f, g| {
                sumcheck_deg2_eq_projective_different_claims_bn254_upper(f, g, &eq, &limbs)
            }),
            expected
        );
        assert_eq!(
            run(&f, &g, |f, g| {
                sumcheck_deg2_eq_projective_different_claims_delayed_bn254_upper(f, g, &eq, &limbs)
            }),
            expected
        );
    }
}

// Logical field-operation counts include evaluation, accumulation and binding;
// suffix construction is outside the measured/counting region.
#[derive(Copy, Clone)]
struct Counted(u64);
thread_local! { static OPS: std::cell::Cell<(usize,usize,usize)> = const { std::cell::Cell::new((0,0,0)) }; }
impl Add for Counted {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        OPS.with(|v| {
            let (m, a, s) = v.get();
            v.set((m, a + 1, s));
        });
        Self(self.0.wrapping_add(rhs.0))
    }
}
impl AddAssign for Counted {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Counted {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        OPS.with(|v| {
            let (m, a, s) = v.get();
            v.set((m, a, s + 1));
        });
        Self(self.0.wrapping_sub(rhs.0))
    }
}
impl Mul for Counted {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        OPS.with(|v| {
            let (m, a, s) = v.get();
            v.set((m + 1, a, s));
        });
        Self(self.0.wrapping_mul(rhs.0))
    }
}
#[test]
fn logical_operation_counts() {
    for n in 0..7 {
        let size = 1usize << n;
        let (mut f, mut g) = (vec![Counted(3); size], vec![Counted(5); size]);
        let ch = vec![Counted(7); n];
        let eq = build_suffix_eq_tables_projective(&vec![Counted(11); n], Counted(1));
        OPS.with(|v| v.set((0, 0, 0)));
        sumcheck_deg2_eq_projective_different_claims(&mut f, &mut g, &eq, &ch, Counted(0));
        let expected = if n == 0 {
            (0, 0, 0)
        } else {
            (5 * size - 4, 6 * (size - 1), 0)
        };
        assert_eq!(OPS.with(|v| v.get()), expected, "n={n}");
    }
}
