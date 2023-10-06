use criterion::{black_box, criterion_group, criterion_main, Criterion};
use synth_brute::{utils::test_data::*, *};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("apply 4 placements in 5x5 cauldron", |b| {
        let materials = vec![
            vec![material::uni(), material::uni()],
            vec![material::beehive()],
            vec![material::broken_stone()],
        ];
        let placements = [
            ((0, 0), Placement::new(2 + 5, None)),
            ((1, 0), Placement::new(1 + 5 * 3, None)),
            ((2, 0), Placement::new(3 + 5 * 2, None)),
            ((0, 1), Placement::new(0, None)),
        ];
        b.iter(|| {
            let mut cauldron = cauldron::uni_bag_5x5_bonus1();
            let mut scores = vec![ColorScoreSet::default(); materials.len()];

            for (item, placement) in placements {
                cauldron
                    .place(&materials, item, placement, &mut scores)
                    .unwrap();
            }
        })
    });

    c.bench_function("optimal routes for uni bag on basic 5x5 cauldron", |b| {
        let materials = vec![
            vec![material::uni(), material::uni()],
            vec![material::beehive()],
            vec![material::broken_stone()],
        ];
        let cauldron = cauldron::uni_bag_5x5_bonus1();
        let goals = goals::uni_bag();
        b.iter(|| {
            black_box(find_optimal::find_optimal_routes(
                black_box(&cauldron),
                black_box(&materials),
                black_box(&goals),
            ));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
