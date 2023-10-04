use criterion::{black_box, criterion_group, criterion_main, Criterion};
use synth_brute::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("apply 4 placements in 5x5 cauldron", |b| {
        let materials = get_input_materials();
        let placements = [
            ((0, 0), Placement::new(2 + 5, ())),
            ((1, 0), Placement::new(1 + 5 * 3, ())),
            ((2, 0), Placement::new(3 + 5 * 2, ())),
            ((0, 1), Placement::new(0, ())),
        ];
        b.iter(|| {
            let mut cauldron = get_test_cauldron();
            let mut scores = vec![ColorScoreSet::default(); materials.len()];

            for (item, placement) in placements {
                cauldron
                    .place(&materials, item, placement, &mut scores)
                    .unwrap();
            }
        })
    });

    c.bench_function("optimal routes for uni bag on basic 5x5 cauldron", |b| {
        let materials = get_input_materials();
        let cauldron = get_test_cauldron();
        let goals = get_uni_bag_goals();
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

macro_rules! tiles {
    ($($color:ident $level:expr,)*) => {
        vec![
            $(tile!($color $level),)*
        ]
    };
}

macro_rules! tile {
    (R $level:expr) => {tile!(Red $level)};
    (B $level:expr) => {tile!(Blue $level)};
    (G $level:expr) => {tile!(Green $level)};
    (Y $level:expr) => {tile!(Yellow $level)};
    (W $level:expr) => {tile!(White $level)};
    ($color:ident $level:expr) => {
        Some(Tile {
            color: Color::$color,
            level: $level,
            played_color: None,
        })
    };
    (None 0) => {
        None
    };
}

fn get_test_cauldron() -> Cauldron {
    Cauldron {
        size: 5,
        tiles: tiles![
            B 0, G 0, Y 0, Y 0, W 0,
            W 0, Y 0, Y 0, Y 0, Y 1,
            R 0, Y 0, R 1, R 0, Y 0,
            R 0, Y 0, R 0, R 0, Y 1,
            W 0, Y 2, Y 0, Y 0, Y 0,
        ],
    }
}

fn get_input_materials() -> Vec<Vec<Material>> {
    // 2 uni, 1 beehive and 1 broken stone
    vec![
        vec![
            Material {
                color: Color::Yellow,
                effect_value: 15,
                shape: Shape::from_binary([0b100, 0b100, 0b100]),
            };
            2
        ],
        vec![Material {
            color: Color::Yellow,
            effect_value: 10,
            shape: Shape::from_binary([0b100, 0b110, 0b000]),
        }],
        vec![Material {
            color: Color::White,
            effect_value: 15,
            shape: Shape::from_binary([0b100, 0b100, 0b100]),
        }],
    ]
}

fn get_uni_bag_goals() -> [Goal; 3] {
    [
        Goal {
            effect_value_thresholds: vec![50, 100],
        },
        Goal {
            effect_value_thresholds: vec![30, 50],
        },
        Goal {
            effect_value_thresholds: vec![30, 55],
        },
    ]
}
