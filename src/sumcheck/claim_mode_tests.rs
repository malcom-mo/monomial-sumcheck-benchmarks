//! Direct contraction checks for every claim mode, field and benchmark kernel.
use super::*;
use binius_field::Field as _;
use jolt_field::Zero as _;

#[derive(Copy, Clone)]
enum Basis {
    Boolean,
    Projective,
    OneInf,
    Flipped,
}

fn run<F: Copy, M: Copy + 'static>(
    f: &[F],
    g: &[F],
    kernel: impl FnOnce(&mut Vec<F>, &mut Vec<F>),
) -> (Vec<M>, Vec<F>, Vec<F>) {
    let (mut f, mut g) = (f.to_vec(), g.to_vec());
    let messages = round_messages_of(|| kernel(&mut f, &mut g));
    (messages, f, g)
}

fn restrict<F>(table: &[F], point: &[F], basis: Basis, reversed: bool, zero: F, one: F) -> Vec<F>
where
    F: Copy + AddAssign + Sub<Output = F> + Mul<Output = F>,
{
    table
        .chunks_exact(1 << point.len())
        .map(|chunk| {
            let mut sum = zero;
            for (i, &value) in chunk.iter().enumerate() {
                let mut term = value;
                for (k, &r) in point.iter().enumerate() {
                    let bit = i >> k & 1 != 0;
                    let factor = match basis {
                        Basis::Boolean => {
                            if bit {
                                r
                            } else {
                                one - r
                            }
                        }
                        Basis::OneInf => {
                            if bit {
                                r - one
                            } else {
                                one
                            }
                        }
                        _ => {
                            if bit != reversed {
                                r
                            } else {
                                one
                            }
                        }
                    };
                    term = term * factor;
                }
                sum += term;
            }
            sum
        })
        .collect()
}

fn weight<F: Copy + Sub<Output = F> + Mul<Output = F>>(
    a: &[F],
    i: usize,
    boolean: bool,
    one: F,
) -> F {
    a.iter().enumerate().fold(one, |w, (k, &a)| {
        w * if i >> k & 1 != 0 {
            a
        } else if boolean {
            one - a
        } else {
            one
        }
    })
}

// Reference sums use the original inputs and direct tensor contraction rather
// than the optimized destructive loops or their suffix weight tables.
fn reference<F>(
    f: &[F],
    g: &[F],
    a: &[F],
    ch: &[F],
    basis: Basis,
    weighted: bool,
    different: bool,
    zero: F,
    one: F,
) -> (Vec<(F, F, F)>, Vec<F>, Vec<F>)
where
    F: Copy
        + Add<Output = F>
        + AddAssign
        + Sub<Output = F>
        + Mul<Output = F>
        + PartialEq
        + std::fmt::Debug,
{
    let initial_f: Vec<_> = f
        .iter()
        .enumerate()
        .map(|(i, &f)| {
            if different {
                f * weight(a, i, false, one)
            } else {
                f
            }
        })
        .collect();
    let reversed = matches!(basis, Basis::Flipped);
    let mut messages = Vec::new();
    for k in 0..ch.len() {
        let fr = restrict(&initial_f, &ch[..k], basis, false, zero, one);
        let gr = restrict(g, &ch[..k], basis, reversed, zero, one);
        let (mut h0, mut h1, mut hi) = (zero, zero, zero);
        for (j, (fp, gp)) in fr.chunks_exact(2).zip(gr.chunks_exact(2)).enumerate() {
            let w = if weighted && !different {
                weight(&a[k + 1..], j, matches!(basis, Basis::Boolean), one)
            } else {
                one
            };
            let (v0, v1, vi) = match basis {
                Basis::Boolean => (
                    fp[0] * gp[0],
                    fp[1] * gp[1],
                    (fp[1] - fp[0]) * (gp[1] - gp[0]),
                ),
                Basis::OneInf => (
                    (fp[0] - fp[1]) * (gp[0] - gp[1]),
                    fp[0] * gp[0],
                    fp[1] * gp[1],
                ),
                Basis::Projective => (
                    fp[0] * gp[0],
                    (fp[0] + fp[1]) * (gp[0] + gp[1]),
                    fp[1] * gp[1],
                ),
                Basis::Flipped => (
                    fp[0] * gp[1],
                    (fp[0] + fp[1]) * (gp[0] + gp[1]),
                    fp[1] * gp[0],
                ),
            };
            h0 += w * v0;
            h1 += w * v1;
            hi += w * vi;
        }
        messages.push((h0, h1, hi));
    }
    let ff = restrict(&initial_f, ch, basis, false, zero, one);
    let gg = restrict(g, ch, basis, reversed, zero, one);
    if different {
        let ar: Vec<_> = a.iter().zip(ch).map(|(&a, &r)| a * r).collect();
        assert_eq!(ff, restrict(f, &ar, Basis::Projective, false, zero, one));
    }
    (messages, ff, gg)
}

macro_rules! check_case {
    ($field:ty,$message:ty,$normalize:expr,$f:ident,$g:ident,$a:ident,$ch:ident,$zero:expr,$one:expr,$basis:expr,$weighted:expr,$different:expr,$kernel:ident,[$($arg:expr),*]) => {
        paste::paste! {
            let expected=reference(&$f,&$g,&$a,&$ch,$basis,$weighted,$different,$zero,$one);
            let normalize=$normalize;
            let triples:Vec<_>=expected.0.iter().map(|&(x,y,z)|(normalize(x),normalize(y),normalize(z))).collect();
            let pairs:Vec<_>=triples.iter().map(|&(x,y,z)| {
                if matches!($basis,Basis::Flipped|Basis::OneInf) || (matches!($basis,Basis::Boolean) && !$weighted) {(x,z)}else{(y,z)}
            }).collect();
            let aware=run::<$field,($message,$message)>(&$f,&$g,|f,g|[<$kernel _claim_aware>](f,g,$($arg),*));
            let unaware=run::<$field,($message,$message,$message)>(&$f,&$g,|f,g|[<$kernel _claim_unaware>](f,g,$($arg),*));
            assert_eq!(aware,(pairs,expected.1.clone(),expected.2.clone()),concat!(stringify!($kernel)," aware"));
            assert_eq!(unaware,(triples,expected.1,expected.2),concat!(stringify!($kernel)," unaware"));
        }
    };
}

#[test]
fn bn254_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (make_bn254(1 << n), make_bn254(1 << n));
        let mixed: Vec<_> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    BN254Fr::ZERO
                } else {
                    BN254Fr::from(1u64)
                }
            })
            .collect();
        for challenge_kind in 0..3 {
            let ch = match challenge_kind {
                0 => make_bn254(n),
                1 => vec![BN254Fr::ZERO; n],
                _ => vec![BN254Fr::from(1u64); n],
            };
            for a in [
                make_bn254(n),
                vec![BN254Fr::ZERO; n],
                vec![BN254Fr::from(1u64); n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, BN254Fr::from(1u64));
                let projective_weights = build_suffix_eq_tables_projective(&a, BN254Fr::from(1u64));
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(BN254Fr,BN254Fr,|x: BN254Fr| x,f,g,a,ch,BN254Fr::ZERO,BN254Fr::from(1u64),Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean,[&ch, BN254Fr::ZERO]),
                (Boolean,false,false,sumcheck_deg2_delayed_bn254,[&ch]),
                (Projective,false,false,sumcheck_deg2_projective,[&ch, BN254Fr::ZERO]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_bn254,[&ch]),
                (Flipped,false,false,sumcheck_deg2_flipped,[&ch, BN254Fr::ZERO]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_bn254,[&ch]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean,[&boolean_weights, &ch, BN254Fr::ZERO]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_bn254,[&boolean_weights, &ch]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective,[&projective_weights, &ch, BN254Fr::ZERO]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_bn254,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims,[&projective_weights, &ch, BN254Fr::ZERO]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_bn254,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims,[&projective_weights, &ch, BN254Fr::ZERO]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_bn254,[&projective_weights, &ch]),
                }
            }
        }
    }
}

#[test]
fn bn254_branch_free_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (
            make_bn254_branch_free(1 << n),
            make_bn254_branch_free(1 << n),
        );
        let mixed: Vec<_> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    Bn254BranchFree::ZERO
                } else {
                    Bn254BranchFree::ONE
                }
            })
            .collect();
        for challenge_kind in 0..3 {
            let ch = match challenge_kind {
                0 => make_bn254_branch_free(n),
                1 => vec![Bn254BranchFree::ZERO; n],
                _ => vec![Bn254BranchFree::ONE; n],
            };
            for a in [
                make_bn254_branch_free(n),
                vec![Bn254BranchFree::ZERO; n],
                vec![Bn254BranchFree::ONE; n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, Bn254BranchFree::ONE);
                let projective_weights =
                    build_suffix_eq_tables_projective(&a, Bn254BranchFree::ONE);
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(Bn254BranchFree,Bn254BranchFree,|x: Bn254BranchFree| x,f,g,a,ch,Bn254BranchFree::ZERO,Bn254BranchFree::ONE,Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean,[&ch, Bn254BranchFree::ZERO]),
                (Projective,false,false,sumcheck_deg2_projective,[&ch, Bn254BranchFree::ZERO]),
                (Flipped,false,false,sumcheck_deg2_flipped,[&ch, Bn254BranchFree::ZERO]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean,[&boolean_weights, &ch, Bn254BranchFree::ZERO]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective,[&projective_weights, &ch, Bn254BranchFree::ZERO]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims,[&projective_weights, &ch, Bn254BranchFree::ZERO]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims,[&projective_weights, &ch, Bn254BranchFree::ZERO]),
                }
                macro_rules! cases_1 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(Bn254BranchFree,BN254Fr,|x: Bn254BranchFree| x.0,f,g,a,ch,Bn254BranchFree::ZERO,Bn254BranchFree::ONE,Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_1! {
                (Boolean,false,false,sumcheck_deg2_delayed_bn254_generic,[&ch]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_bn254_generic,[&ch]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_bn254_generic,[&ch]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_bn254_generic,[&boolean_weights, &ch]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_bn254_generic,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_bn254_generic,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_bn254_generic,[&projective_weights, &ch]),
                }
            }
        }
    }
}

#[test]
fn bn254_upper_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (make_bn254(1 << n), make_bn254(1 << n));
        let mixed: Vec<_> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    BN254Fr::ZERO
                } else {
                    BN254Fr::from(1u64)
                }
            })
            .collect();
        for challenge_kind in 0..3 {
            let (mut ch, mut limbs) = make_bn254_upper_limb_challenges(n);
            if challenge_kind == 1 {
                ch.fill(BN254Fr::ZERO);
                limbs.fill((0, 0));
            }
            if challenge_kind == 2 && n > 0 {
                ch[0] = BN254Fr::ZERO;
                limbs[0] = (0, 0);
            }
            for a in [
                make_bn254(n),
                vec![BN254Fr::ZERO; n],
                vec![BN254Fr::from(1u64); n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, BN254Fr::from(1u64));
                let projective_weights = build_suffix_eq_tables_projective(&a, BN254Fr::from(1u64));
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(BN254Fr,BN254Fr,|x: BN254Fr| x,f,g,a,ch,BN254Fr::ZERO,BN254Fr::from(1u64),Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean_bn254_upper,[&limbs]),
                (Boolean,false,false,sumcheck_deg2_delayed_bn254_upper,[&limbs]),
                (Projective,false,false,sumcheck_deg2_projective_bn254_upper,[&limbs]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_bn254_upper,[&limbs]),
                (Flipped,false,false,sumcheck_deg2_flipped_bn254_upper,[&limbs]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_bn254_upper,[&limbs]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean_bn254_upper,[&boolean_weights, &limbs]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_bn254_upper,[&boolean_weights, &limbs]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective_bn254_upper,[&projective_weights, &limbs]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_bn254_upper,[&projective_weights, &limbs]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_bn254_upper,[&projective_weights, &limbs]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_bn254_upper,[&projective_weights, &limbs]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_bn254_upper,[&projective_weights, &limbs]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_bn254_upper,[&projective_weights, &limbs]),
                }
            }
        }
    }
}

#[test]
fn bb4_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (make_bb4(1 << n), make_bb4(1 << n));
        let mixed: Vec<_> = (0..n)
            .map(|i| if i % 2 == 0 { BB4::ZERO } else { BB4::ONE })
            .collect();
        for challenge_kind in 0..3 {
            let ch = match challenge_kind {
                0 => make_bb4(n),
                1 => vec![BB4::ZERO; n],
                _ => vec![BB4::ONE; n],
            };
            for a in [
                make_bb4(n),
                vec![BB4::ZERO; n],
                vec![BB4::ONE; n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, BB4::ONE);
                let projective_weights = build_suffix_eq_tables_projective(&a, BB4::ONE);
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(BB4,BB4,|x: BB4| x,f,g,a,ch,BB4::ZERO,BB4::ONE,Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean,[&ch, BB4::ZERO]),
                (Boolean,false,false,sumcheck_deg2_delayed_bb4,[&ch]),
                (Projective,false,false,sumcheck_deg2_projective,[&ch, BB4::ZERO]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_bb4,[&ch]),
                (Flipped,false,false,sumcheck_deg2_flipped,[&ch, BB4::ZERO]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_bb4,[&ch]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean,[&boolean_weights, &ch, BB4::ZERO]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_bb4,[&boolean_weights, &ch]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective,[&projective_weights, &ch, BB4::ZERO]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_bb4,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims,[&projective_weights, &ch, BB4::ZERO]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_bb4,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims,[&projective_weights, &ch, BB4::ZERO]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_bb4,[&projective_weights, &ch]),
                }
            }
        }
    }
}

#[test]
fn bb5_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (make_bb5(1 << n), make_bb5(1 << n));
        let mixed: Vec<_> = (0..n)
            .map(|i| if i % 2 == 0 { BB5::ZERO } else { BB5::ONE })
            .collect();
        for challenge_kind in 0..3 {
            let ch = match challenge_kind {
                0 => make_bb5(n),
                1 => vec![BB5::ZERO; n],
                _ => vec![BB5::ONE; n],
            };
            for a in [
                make_bb5(n),
                vec![BB5::ZERO; n],
                vec![BB5::ONE; n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, BB5::ONE);
                let projective_weights = build_suffix_eq_tables_projective(&a, BB5::ONE);
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(BB5,BB5,|x: BB5| x,f,g,a,ch,BB5::ZERO,BB5::ONE,Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean,[&ch, BB5::ZERO]),
                (Boolean,false,false,sumcheck_deg2_delayed_bb5,[&ch]),
                (Projective,false,false,sumcheck_deg2_projective,[&ch, BB5::ZERO]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_bb5,[&ch]),
                (Flipped,false,false,sumcheck_deg2_flipped,[&ch, BB5::ZERO]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_bb5,[&ch]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean_bb5,[&boolean_weights, &ch, BB5::ZERO]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_bb5_packed,[&boolean_weights, &ch]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective_bb5,[&projective_weights, &ch, BB5::ZERO]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_bb5_packed,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_bb5,[&projective_weights, &ch, BB5::ZERO]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_bb5,[&projective_weights, &ch, BB5::ZERO]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_bb5,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_bb5,[&projective_weights, &ch]),
                }
            }
        }
    }
}

#[test]
fn kb5_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (make_kb5(1 << n), make_kb5(1 << n));
        let mixed: Vec<_> = (0..n)
            .map(|i| if i % 2 == 0 { KB5::ZERO } else { KB5::ONE })
            .collect();
        for challenge_kind in 0..3 {
            let ch = match challenge_kind {
                0 => make_kb5(n),
                1 => vec![KB5::ZERO; n],
                _ => vec![KB5::ONE; n],
            };
            for a in [
                make_kb5(n),
                vec![KB5::ZERO; n],
                vec![KB5::ONE; n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, KB5::ONE);
                let projective_weights = build_suffix_eq_tables_projective(&a, KB5::ONE);
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(KB5,KB5,|x: KB5| x,f,g,a,ch,KB5::ZERO,KB5::ONE,Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean,[&ch, KB5::ZERO]),
                (Boolean,false,false,sumcheck_deg2_delayed_kb5,[&ch]),
                (Projective,false,false,sumcheck_deg2_projective,[&ch, KB5::ZERO]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_kb5,[&ch]),
                (Flipped,false,false,sumcheck_deg2_flipped,[&ch, KB5::ZERO]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_kb5,[&ch]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean,[&boolean_weights, &ch, KB5::ZERO]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_kb5,[&boolean_weights, &ch]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective,[&projective_weights, &ch, KB5::ZERO]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_kb5,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims,[&projective_weights, &ch, KB5::ZERO]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_kb5,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims,[&projective_weights, &ch, KB5::ZERO]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_kb5,[&projective_weights, &ch]),
                }
            }
        }
    }
}

#[test]
fn fp128_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (make_fp128(1 << n), make_fp128(1 << n));
        let mixed: Vec<_> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    Fp128::zero()
                } else {
                    Fp128::one()
                }
            })
            .collect();
        for challenge_kind in 0..3 {
            let ch = match challenge_kind {
                0 => make_fp128(n),
                1 => vec![Fp128::zero(); n],
                _ => vec![Fp128::one(); n],
            };
            for a in [
                make_fp128(n),
                vec![Fp128::zero(); n],
                vec![Fp128::one(); n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, Fp128::one());
                let projective_weights = build_suffix_eq_tables_projective(&a, Fp128::one());
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(Fp128,Fp128,|x: Fp128| x,f,g,a,ch,Fp128::zero(),Fp128::one(),Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean_fp128,[&ch, Fp128::zero()]),
                (Boolean,false,false,sumcheck_deg2_delayed_fp128,[&ch]),
                (Projective,false,false,sumcheck_deg2_projective_fp128,[&ch, Fp128::zero()]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_fp128,[&ch]),
                (Flipped,false,false,sumcheck_deg2_flipped_fp128,[&ch, Fp128::zero()]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_fp128,[&ch]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean_fp128,[&boolean_weights, &ch, Fp128::zero()]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_fp128,[&boolean_weights, &ch]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective_fp128,[&projective_weights, &ch, Fp128::zero()]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_fp128,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_fp128,[&projective_weights, &ch, Fp128::zero()]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_fp128,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_fp128,[&projective_weights, &ch, Fp128::zero()]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_fp128,[&projective_weights, &ch]),
                (OneInf,false,false,sumcheck_deg2_projective_1inf_fp128,[&ch, Fp128::zero()]),
                (OneInf,false,false,sumcheck_deg2_projective_1inf_delayed_fp128,[&ch]),
                (OneInf,true,false,sumcheck_deg2_eq_gruen_projective_1inf_fp128,[&projective_weights, &ch, Fp128::zero()]),
                (OneInf,true,false,sumcheck_deg2_eq_projective_1inf_delayed_fp128,[&projective_weights, &ch]),
                }
            }
        }
    }
}

#[test]
fn gf128_claim_modes() {
    for n in [0, 1, 2, 3, 6] {
        let (f, g) = (make_gf128(1 << n), make_gf128(1 << n));
        let mixed: Vec<_> = (0..n)
            .map(|i| if i % 2 == 0 { GF128::ZERO } else { GF128::ONE })
            .collect();
        for challenge_kind in 0..3 {
            let ch = match challenge_kind {
                0 => make_gf128(n),
                1 => vec![GF128::ZERO; n],
                _ => vec![GF128::ONE; n],
            };
            for a in [
                make_gf128(n),
                vec![GF128::ZERO; n],
                vec![GF128::ONE; n],
                mixed.clone(),
            ] {
                let boolean_weights = build_suffix_eq_tables(&a, GF128::ONE);
                let projective_weights = build_suffix_eq_tables_projective(&a, GF128::ONE);
                macro_rules! cases_0 {
 ($(($basis:ident,$weighted:literal,$different:literal,$kernel:ident,[$($arg:expr),*])),* $(,)?) => {
$(check_case!(GF128,GF128,|x: GF128| x,f,g,a,ch,GF128::ZERO,GF128::ONE,Basis::$basis,$weighted,$different,$kernel,[$($arg),*]);)*
 };
}
                cases_0! {
                (Boolean,false,false,sumcheck_deg2_boolean,[&ch, GF128::ZERO]),
                (Boolean,false,false,sumcheck_deg2_delayed_gf128,[&ch]),
                (Projective,false,false,sumcheck_deg2_projective,[&ch, GF128::ZERO]),
                (Projective,false,false,sumcheck_deg2_projective_delayed_gf128,[&ch]),
                (Flipped,false,false,sumcheck_deg2_flipped,[&ch, GF128::ZERO]),
                (Flipped,false,false,sumcheck_deg2_flipped_delayed_gf128,[&ch]),
                (Boolean,true,false,sumcheck_deg2_eq_gruen_boolean,[&boolean_weights, &ch, GF128::ZERO]),
                (Boolean,true,false,sumcheck_deg2_eq_delayed_gf128,[&boolean_weights, &ch]),
                (Projective,true,false,sumcheck_deg2_eq_gruen_projective,[&projective_weights, &ch, GF128::ZERO]),
                (Projective,true,false,sumcheck_deg2_eq_projective_delayed_gf128,[&projective_weights, &ch]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims,[&projective_weights, &ch, GF128::ZERO]),
                (Projective,true,true,sumcheck_deg2_eq_projective_different_claims_delayed_gf128,[&projective_weights, &ch]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims,[&projective_weights, &ch, GF128::ZERO]),
                (Flipped,true,true,sumcheck_deg2_eq_flipped_different_claims_delayed_gf128,[&projective_weights, &ch]),
                }
            }
        }
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
fn operation_counts_for_both_claim_modes() {
    for n in 0..7 {
        let size = 1usize << n;
        let f = vec![Counted(3); size];
        let g = vec![Counted(5); size];
        let ch = vec![Counted(7); n];
        let eq = build_suffix_eq_tables_projective(&vec![Counted(11); n], Counted(1));
        let p = size - 1;
        for aware in [true, false] {
            for which in 0..7 {
                let (mut f, mut g) = (f.clone(), g.clone());
                OPS.with(|v| v.set((0, 0, 0)));
                macro_rules! invoke {
                    ($mode:literal) => {
                        match which {
                            0 => sumcheck_deg2_boolean_with_claim_mode::<$mode, _>(
                                &mut f,
                                &mut g,
                                &ch,
                                Counted(0),
                            ),
                            1 => sumcheck_deg2_projective_with_claim_mode::<$mode, _>(
                                &mut f,
                                &mut g,
                                &ch,
                                Counted(0),
                            ),
                            2 => sumcheck_deg2_flipped_with_claim_mode::<$mode, _>(
                                &mut f,
                                &mut g,
                                &ch,
                                Counted(0),
                            ),
                            3 => sumcheck_deg2_eq_gruen_boolean_with_claim_mode::<$mode, _>(
                                &mut f,
                                &mut g,
                                &eq,
                                &ch,
                                Counted(0),
                            ),
                            4 => sumcheck_deg2_eq_gruen_projective_with_claim_mode::<$mode, _>(
                                &mut f,
                                &mut g,
                                &eq,
                                &ch,
                                Counted(0),
                            ),
                            5 => sumcheck_deg2_eq_projective_different_claims_with_claim_mode::<
                                $mode,
                                _,
                            >(&mut f, &mut g, &eq, &ch, Counted(0)),
                            _ => sumcheck_deg2_eq_flipped_different_claims_with_claim_mode::<
                                $mode,
                                _,
                            >(&mut f, &mut g, &eq, &ch, Counted(0)),
                        }
                    };
                }
                if aware {
                    invoke!(true);
                } else {
                    invoke!(false);
                }
                let third = usize::from(!aware);
                let m = match which {
                    0..=2 => (4 + third) * p,
                    3 | 4 => (6 + third) * p,
                    _ => {
                        if n == 0 {
                            0
                        } else {
                            (5 + third) * size - (4 + third)
                        }
                    }
                };
                let a = match which {
                    0 | 3 => (4 + third) * p,
                    1 | 4 | 5 => (6 + third) * p,
                    _ => {
                        if aware {
                            4 * p
                        } else {
                            7 * p
                        }
                    }
                };
                let sub = if which == 0 || which == 3 { 4 * p } else { 0 };
                assert_eq!(
                    OPS.with(|v| v.get()),
                    (m, a, sub),
                    "n={n}, aware={aware}, kernel={which}"
                );
            }
        }
    }
}
