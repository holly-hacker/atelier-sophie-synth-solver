use criterion::{black_box, criterion_group, criterion_main, Criterion};

use synth_solver::{solver::SolverSettings, utils::test_data::*, *};

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
                    .place(&materials, item, placement, false, &mut scores)
                    .unwrap();
            }
        });
    });

    c.bench_function("optimal routes for uni bag on basic 5x5 cauldron", |b| {
        let materials = vec![
            vec![material::uni(), material::uni()],
            vec![material::beehive()],
            vec![material::broken_stone()],
        ];
        let cauldron = cauldron::uni_bag_5x5_bonus1();
        let goals = goals::uni_bag();
        let properties = SolverSettings::default();
        b.iter(|| {
            black_box(solver::find_optimal_routes(
                black_box(&cauldron),
                black_box(&materials),
                black_box(&goals),
                black_box(&properties),
                None,
            ));
        });
    });

    c.bench_function("find optimal routes with perfect solution", |b| {
        let perfect_material = Material {
            color: Color::White,
            effect_value: 1000,
            shape: Shape::from_binary([0b100, 0b100, 0b100]),
        };
        let materials = vec![
            vec![perfect_material, material::uni()],
            vec![perfect_material, material::beehive()],
            vec![perfect_material, material::broken_stone()],
        ];
        let cauldron = cauldron::uni_bag_5x5_bonus1();
        let goals = goals::uni_bag();
        let properties = SolverSettings::default();
        b.iter(|| {
            black_box(solver::find_optimal_routes(
                black_box(&cauldron),
                black_box(&materials),
                black_box(&goals),
                black_box(&properties),
                None,
            ));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
