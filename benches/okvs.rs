use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::collections::HashSet;

use fuzzy_psi::okvs::lagrange::{LagrangePolynomialOKVS, Point as LagrangePoint};
use fuzzy_psi::okvs::near_optimal::okvs::{Okvs, OkvsKey, OkvsValue, RbOkvs};

fn lagrange_points(n: usize) -> HashSet<LagrangePoint> {
    (1..=n as u64).map(|i| LagrangePoint::new(i, i)).collect()
}

fn rb_pairs(n: usize) -> Vec<(OkvsKey, OkvsValue)> {
    (1..=n as u64)
        .map(|i| (OkvsKey(i.to_le_bytes()), OkvsValue(i.to_le_bytes())))
        .collect()
}

fn bench_okvs(c: &mut Criterion) {
    let mut group = c.benchmark_group("OKVS Compare");
    group.sample_size(10);
    let sizes = [64, 128, 256, 512];

    for &size in &sizes {
        // LagrangePolynomialOKVS
        group.bench_function(format!("Lagrange encode n={}", size), |b| {
            let data = lagrange_points(size);
            b.iter_batched(
                || data.clone(),
                |d| {
                    LagrangePolynomialOKVS::encode(&d);
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_function(format!("Lagrange decode n={}", size), |b| {
            let data = lagrange_points(size);
            let okvs = LagrangePolynomialOKVS::encode(&data);
            let keys: Vec<u64> = data.iter().map(|p| p.x).collect();
            b.iter(|| {
                for &k in &keys {
                    let _ = okvs.decode(k);
                }
            });
        });

        // RbOkvs (near-optimal)
        group.bench_function(format!("RbOkvs encode n={}", size), |b| {
            let data = rb_pairs(size);
            let rb = RbOkvs::new(size);
            b.iter_batched(
                || data.clone(),
                |d| {
                    let _ = rb.encode(d).unwrap();
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_function(format!("RbOkvs decode n={}", size), |b| {
            let data = rb_pairs(size);
            let rb = RbOkvs::new(size);
            let encoding = rb.encode(data.clone()).unwrap();
            let keys: Vec<_> = data.iter().map(|(k, _)| k.clone()).collect();
            b.iter(|| {
                for k in &keys {
                    let _ = rb.decode(&encoding, k);
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_okvs);
criterion_main!(benches);
