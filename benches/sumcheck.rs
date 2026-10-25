use ark_ff::AdditiveGroup as _;
use binius_field::Field as _;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use jolt_field::{One as _, Zero as _};
use p3_field::PrimeCharacteristicRing as _;
use rand::seq::SliceRandom;

use monomial_sumcheck_benchmarks::{benchmark_config, sumcheck::*};

// Awareness is selected outside the measured closure. Each wrapper specializes
// a const-generic kernel, so there is no runtime awareness branch in its loops.
macro_rules! register_case {
    ($group:ident, $aware:expr, $label:literal, $n:ident, $f:ident, $g:ident,
     $kernel:ident, [$($arg:expr),*]) => {
        paste::paste! {
            if $aware {
                $group.bench_with_input(BenchmarkId::new(concat!($label, "-claim-aware"), $n), &$n, |b, _| {
                    b.iter_batched(|| ($f.clone(), $g.clone()), |(mut f, mut g)| {
                        [<$kernel _claim_aware>](&mut f, &mut g, $($arg),*);
                        let _ = criterion::black_box((f[0], g[0]));
                    }, criterion::BatchSize::LargeInput)
                });
            } else {
                $group.bench_with_input(BenchmarkId::new(concat!($label, "-claim-unaware"), $n), &$n, |b, _| {
                    b.iter_batched(|| ($f.clone(), $g.clone()), |(mut f, mut g)| {
                        [<$kernel _claim_unaware>](&mut f, &mut g, $($arg),*);
                        let _ = criterion::black_box((f[0], g[0]));
                    }, criterion::BatchSize::LargeInput)
                });
            }
        }
    };
}

macro_rules! field_benches {
    ($bench:ident, $label:literal, $make:ident, $make_challenges:expr, $one:expr, $ns:expr,
     |$ch:ident, $boolean_weights:ident, $projective_weights:ident|
     unweighted [$(($unweighted_label:literal, $unweighted:ident, [$($uarg:expr),*])),* $(,)?]
     weighted [$(($weighted_label:literal, $weighted:ident, [$($warg:expr),*])),* $(,)?]) => {
        fn $bench(c: &mut Criterion) {
            {
                let mut group = c.benchmark_group(concat!("sumcheck_deg2/", $label));
                for &n in &$ns {
                    let f_orig = $make(1usize << n);
                    let g_orig = $make(1usize << n);
                    let $ch = ($make_challenges)(n as usize);
                    let labels = [$($unweighted_label),*];
                    let mut order: Vec<_> = (0..2*labels.len()).collect();
                    order.shuffle(&mut rand::thread_rng());
                    for case in order {
                        let aware = case % 2 == 0;
                        $(if labels[case/2] == $unweighted_label {
                            register_case!(group, aware, $unweighted_label, n, f_orig, g_orig, $unweighted, [$($uarg),*]);
                        })*
                    }
                }
                group.finish();
            }
            {
                let mut group = c.benchmark_group(concat!("sumcheck_deg2_eq/", $label));
                for &n in &$ns {
                    let f_orig = $make(1usize << n);
                    let g_orig = $make(1usize << n);
                    let $ch = ($make_challenges)(n as usize);
                    let point = $make(n as usize);
                    let $boolean_weights = build_suffix_eq_tables(&point, $one);
                    let $projective_weights = build_suffix_eq_tables_projective(&point, $one);
                    let labels = [$($weighted_label),*];
                    let mut order: Vec<_> = (0..2*labels.len()).collect();
                    order.shuffle(&mut rand::thread_rng());
                    for case in order {
                        let aware = case % 2 == 0;
                        $(if labels[case/2] == $weighted_label {
                            register_case!(group, aware, $weighted_label, n, f_orig, g_orig, $weighted, [$($warg),*]);
                        })*
                    }
                }
                group.finish();
            }
        }
    };
}

field_benches!(bench_bn254, "BN254", make_bn254, make_bn254, BN254Fr::from(1u64), [16u32, 20, 24],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean, [&ch, BN254Fr::ZERO]),
  ("delayed", sumcheck_deg2_delayed_bn254, [&ch]),
  ("projective", sumcheck_deg2_projective, [&ch, BN254Fr::ZERO]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_bn254, [&ch]),
  ("flipped", sumcheck_deg2_flipped, [&ch, BN254Fr::ZERO]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_bn254, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean, [&boolean_weights, &ch, BN254Fr::ZERO]),
  ("delayed", sumcheck_deg2_eq_delayed_bn254, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective, [&projective_weights, &ch, BN254Fr::ZERO]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_bn254, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims, [&projective_weights, &ch, BN254Fr::ZERO]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_bn254, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims, [&projective_weights, &ch, BN254Fr::ZERO]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_bn254, [&projective_weights, &ch]),
 ]
);

field_benches!(bench_bn254_branch_free, "BN254_bf", make_bn254_branch_free, make_bn254_branch_free, Bn254BranchFree::ONE, [16u32, 20, 24],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean, [&ch, Bn254BranchFree::ZERO]),
  ("delayed", sumcheck_deg2_delayed_bn254_generic, [&ch]),
  ("projective", sumcheck_deg2_projective, [&ch, Bn254BranchFree::ZERO]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_bn254_generic, [&ch]),
  ("flipped", sumcheck_deg2_flipped, [&ch, Bn254BranchFree::ZERO]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_bn254_generic, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean, [&boolean_weights, &ch, Bn254BranchFree::ZERO]),
  ("delayed", sumcheck_deg2_eq_delayed_bn254_generic, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective, [&projective_weights, &ch, Bn254BranchFree::ZERO]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_bn254_generic, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims, [&projective_weights, &ch, Bn254BranchFree::ZERO]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_bn254_generic, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims, [&projective_weights, &ch, Bn254BranchFree::ZERO]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_bn254_generic, [&projective_weights, &ch]),
 ]
);

field_benches!(bench_bn254_upper, "BN254_upper", make_bn254, |n| make_bn254_upper_limb_challenges(n).1, BN254Fr::from(1u64), [16u32, 20, 24],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean_bn254_upper, [&ch]),
  ("delayed", sumcheck_deg2_delayed_bn254_upper, [&ch]),
  ("projective", sumcheck_deg2_projective_bn254_upper, [&ch]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_bn254_upper, [&ch]),
  ("flipped", sumcheck_deg2_flipped_bn254_upper, [&ch]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_bn254_upper, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean_bn254_upper, [&boolean_weights, &ch]),
  ("delayed", sumcheck_deg2_eq_delayed_bn254_upper, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective_bn254_upper, [&projective_weights, &ch]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_bn254_upper, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims_bn254_upper, [&projective_weights, &ch]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_bn254_upper, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims_bn254_upper, [&projective_weights, &ch]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_bn254_upper, [&projective_weights, &ch]),
 ]
);

field_benches!(bench_bb4, "BB4", make_bb4, make_bb4, BB4::ONE, [16u32, 20],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean, [&ch, BB4::ZERO]),
  ("delayed", sumcheck_deg2_delayed_bb4, [&ch]),
  ("projective", sumcheck_deg2_projective, [&ch, BB4::ZERO]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_bb4, [&ch]),
  ("flipped", sumcheck_deg2_flipped, [&ch, BB4::ZERO]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_bb4, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean, [&boolean_weights, &ch, BB4::ZERO]),
  ("delayed", sumcheck_deg2_eq_delayed_bb4, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective, [&projective_weights, &ch, BB4::ZERO]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_bb4, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims, [&projective_weights, &ch, BB4::ZERO]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_bb4, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims, [&projective_weights, &ch, BB4::ZERO]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_bb4, [&projective_weights, &ch]),
 ]
);

field_benches!(bench_bb5, "BB5", make_bb5, make_bb5, BB5::ONE, [16u32, 20],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean, [&ch, BB5::ZERO]),
  ("delayed", sumcheck_deg2_delayed_bb5, [&ch]),
  ("projective", sumcheck_deg2_projective, [&ch, BB5::ZERO]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_bb5, [&ch]),
  ("flipped", sumcheck_deg2_flipped, [&ch, BB5::ZERO]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_bb5, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean_bb5, [&boolean_weights, &ch, BB5::ZERO]),
  ("delayed", sumcheck_deg2_eq_delayed_bb5_packed, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective_bb5, [&projective_weights, &ch, BB5::ZERO]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_bb5_packed, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims_bb5, [&projective_weights, &ch, BB5::ZERO]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_bb5_packed, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims_bb5, [&projective_weights, &ch, BB5::ZERO]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_bb5_packed, [&projective_weights, &ch]),
 ]
);

field_benches!(bench_kb5, "KB5", make_kb5, make_kb5, KB5::ONE, [16u32, 20],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean, [&ch, KB5::ZERO]),
  ("delayed", sumcheck_deg2_delayed_kb5, [&ch]),
  ("projective", sumcheck_deg2_projective, [&ch, KB5::ZERO]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_kb5, [&ch]),
  ("flipped", sumcheck_deg2_flipped, [&ch, KB5::ZERO]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_kb5, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean, [&boolean_weights, &ch, KB5::ZERO]),
  ("delayed", sumcheck_deg2_eq_delayed_kb5, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective, [&projective_weights, &ch, KB5::ZERO]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_kb5, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims, [&projective_weights, &ch, KB5::ZERO]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_kb5, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims, [&projective_weights, &ch, KB5::ZERO]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_kb5, [&projective_weights, &ch]),
 ]
);

field_benches!(bench_fp128, "Fp128", make_fp128, make_fp128, Fp128::one(), [16u32, 20],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean_fp128, [&ch, Fp128::zero()]),
  ("delayed", sumcheck_deg2_delayed_fp128, [&ch]),
  ("projective", sumcheck_deg2_projective_fp128, [&ch, Fp128::zero()]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_fp128, [&ch]),
  ("flipped", sumcheck_deg2_flipped_fp128, [&ch, Fp128::zero()]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_fp128, [&ch]),
  ("projective_1inf", sumcheck_deg2_projective_1inf_fp128, [&ch, Fp128::zero()]),
  ("projective_1inf_delayed", sumcheck_deg2_projective_1inf_delayed_fp128, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean_fp128, [&boolean_weights, &ch, Fp128::zero()]),
  ("delayed", sumcheck_deg2_eq_delayed_fp128, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective_fp128, [&projective_weights, &ch, Fp128::zero()]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_fp128, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims_fp128, [&projective_weights, &ch, Fp128::zero()]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_fp128, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims_fp128, [&projective_weights, &ch, Fp128::zero()]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_fp128, [&projective_weights, &ch]),
  ("projective_1inf", sumcheck_deg2_eq_gruen_projective_1inf_fp128, [&projective_weights, &ch, Fp128::zero()]),
  ("projective_1inf_delayed", sumcheck_deg2_eq_projective_1inf_delayed_fp128, [&projective_weights, &ch]),
 ]
);

field_benches!(bench_gf128, "GF128", make_gf128, make_gf128, GF128::ONE, [16u32, 20],
 |ch, boolean_weights, projective_weights|
 unweighted [
  ("boolean", sumcheck_deg2_boolean, [&ch, GF128::ZERO]),
  ("delayed", sumcheck_deg2_delayed_gf128, [&ch]),
  ("projective", sumcheck_deg2_projective, [&ch, GF128::ZERO]),
  ("proj_delayed", sumcheck_deg2_projective_delayed_gf128, [&ch]),
  ("flipped", sumcheck_deg2_flipped, [&ch, GF128::ZERO]),
  ("flipped_delayed", sumcheck_deg2_flipped_delayed_gf128, [&ch]),
 ]
 weighted [
  ("boolean", sumcheck_deg2_eq_gruen_boolean, [&boolean_weights, &ch, GF128::ZERO]),
  ("delayed", sumcheck_deg2_eq_delayed_gf128, [&boolean_weights, &ch]),
  ("projective", sumcheck_deg2_eq_gruen_projective, [&projective_weights, &ch, GF128::ZERO]),
  ("proj_delayed", sumcheck_deg2_eq_projective_delayed_gf128, [&projective_weights, &ch]),
  ("projective-different-claims", sumcheck_deg2_eq_projective_different_claims, [&projective_weights, &ch, GF128::ZERO]),
  ("projective-different-claims-delayed", sumcheck_deg2_eq_projective_different_claims_delayed_gf128, [&projective_weights, &ch]),
  ("flipped-different-claims", sumcheck_deg2_eq_flipped_different_claims, [&projective_weights, &ch, GF128::ZERO]),
  ("flipped-different-claims-delayed", sumcheck_deg2_eq_flipped_different_claims_delayed_gf128, [&projective_weights, &ch]),
 ]
);

criterion_group! {
    name = benches;
    config = benchmark_config();
    targets = bench_bn254, bench_bn254_branch_free, bench_bn254_upper, bench_bb4, bench_bb5, bench_kb5, bench_fp128, bench_gf128
}
criterion_main!(benches);
