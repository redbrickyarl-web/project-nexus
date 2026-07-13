use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use cosmic_queue::ring::CosmicRing;

fn bench_ring(c: &mut Criterion) {
    let mut group = c.benchmark_group("cosmic_ring");
    for cap in [64usize, 256, 1024] {
        group.throughput(Throughput::Elements(cap as u64));
        group.bench_function(format!("push_pop_{}", cap), |b| {
            let ring = CosmicRing::<u64, 1024>::new();
            let mut i = 0u64;
            b.iter(|| {
                let _ = black_box(ring.try_push(black_box(i)));
                let _ = black_box(ring.try_pop());
                i = i.wrapping_add(1);
            });
        });
    }
}

criterion_group!(benches, bench_ring);
criterion_main!(benches);