use criterion::*;
use rand::{RngCore, SeedableRng};

const SEED: [u8; 32] = [
    1, 0, 52, 0, 0, 0, 0, 0, 1, 0, 10, 0, 22, 32, 0, 0, 2, 0, 55, 49, 0, 11, 0, 0, 3, 0, 0, 0, 0,
    0, 2, 92,
];

fn encode_in_okvs(c: &mut Criterion) {
    let mut grp = c.benchmark_group("OKVS::encode");
    for n in [1, 10, 100, 200, 300, 400, 500] {
        let mut rng = rand_chacha::ChaCha20Rng::from_seed(SEED);
        let elements = (1..=n).map(|i| (i, rng.next_u64())).collect::<Vec<_>>();
        match n {
            100 => { grp.sample_size(20); },
            n if n > 100 => { grp.sample_size(10); },
            _ => {},
        }
        grp.throughput(Throughput::Elements(n));
        grp.bench_with_input(BenchmarkId::from_parameter(n), &elements, |b, elements| {
            b.iter(|| {
                let _ = fuzzy_psi::OKVS::encode(elements);
            });
        });
    }
}

fn decode_from_okvs(c: &mut Criterion) {
    let mut rng = rand_chacha::ChaCha20Rng::from_seed(SEED);
    let elements = (1..=500).map(|i| (i, rng.next_u64())).collect::<Vec<_>>();
    println!("encode {} elements.", elements.len());
    let okvs = fuzzy_psi::OKVS::encode(&elements);

    c.bench_function("OKVS::decode", |b| {
        b.iter(|| {
            let _ = okvs.decode(250);
        });
    });
}

criterion_group!(benches, encode_in_okvs, decode_from_okvs);
criterion_main!(benches);
