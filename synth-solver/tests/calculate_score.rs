use synth_solver::*;

#[test]
fn test_calculation_basic() {
    let materials = vec![
        vec![Material::new(Color::Yellow, 15, Shape::from_binary([0b100, 0b100, 0b100])); 2],
        vec![Material::new(
            Color::Yellow,
            10,
            Shape::from_binary([0b100, 0b110, 0b000]),
        )],
        vec![Material::new(
            Color::White,
            15,
            Shape::from_binary([0b100, 0b100, 0b100]),
        )],
    ];

    let mut cauldron = Cauldron {
        size: 5,
        tiles: tiles![
            B 0, G 0, Y 0, Y 0, W 0,
            W 0, Y 0, Y 0, Y 0, Y 1,
            R 0, Y 0, R 1, R 0, Y 0,
            R 0, Y 0, R 0, R 0, Y 1,
            W 0, Y 2, Y 0, Y 0, Y 0,
        ],
        bonus_scores: (3, 5, 7),
        color: Color::White,
        properties: CauldronProperties::empty(),
    };

    let mut scores = vec![ColorScoreSet::default(); materials.len()];

    let placement1 = Placement::new(2 + 5, None);
    cauldron
        .place(&materials, (0, 0), placement1, true, &mut scores)
        .unwrap();

    let placement2 = Placement::new(1 + 5 * 3, None);
    cauldron
        .place(&materials, (1, 0), placement2, true, &mut scores)
        .unwrap();

    let placement3 = Placement::new(3 + 5 * 2, None);
    cauldron
        .place(&materials, (2, 0), placement3, true, &mut scores)
        .unwrap();

    let placement4 = Placement::new(0, None);
    cauldron
        .place(&materials, (0, 1), placement4, true, &mut scores)
        .unwrap();

    let coverage = cauldron.calculate_coverage(&materials);
    assert_eq!(coverage.get_color_ratio(Color::Red, &cauldron), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Blue, &cauldron), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Green, &cauldron), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Yellow, &cauldron), 0.36);
    assert_eq!(coverage.get_color_ratio(Color::White, &cauldron), 0.12);

    let final_scores = cauldron.calculate_final_score(&materials, &scores);

    assert_eq!(final_scores.as_slice(), vec![48, 39, 28]);
}

#[test]
fn test_calculation_overlap_synergy() {
    let materials = vec![
        vec![
            Material::new(Color::Green, 12, Shape::from_binary([0b110, 0b111, 0b000])),
            Material::new(Color::Green, 10, Shape::from_binary([0b100, 0b110, 0b000])),
        ],
        vec![Material::new(
            Color::Yellow,
            22,
            Shape::from_binary([0b010, 0b111, 0b000]),
        )],
        vec![Material::new(
            Color::Green,
            33,
            Shape::from_binary([0b110, 0b001, 0b010]),
        )],
    ];

    let mut cauldron = Cauldron {
        size: 5,
        tiles: tiles![
            B 0, B 0, G 0, G 0, B 0,
            B 0, B 0, G 0, G 1, B 0,
            G 0, G 1, G 0, G 0, G 0,
            G 0, G 0, G 0, G 2, G 0,
            B 0, B 1, G 0, G 0, B 0,
        ],
        bonus_scores: (3, 5, 7),
        color: Color::Green,
        properties: CauldronProperties::SYNERGY,
    };

    let mut scores = vec![ColorScoreSet::default(); materials.len()];

    let placement1 = Placement::new(0, None);
    cauldron
        .place(&materials, (0, 0), placement1, true, &mut scores)
        .unwrap();
    println!("scores after placement 1: {scores:?}");

    let placement2 = Placement::new(2, None);
    cauldron
        .place(&materials, (2, 0), placement2, true, &mut scores)
        .unwrap();
    println!("scores after placement 2: {scores:?}");

    let placement3 = Placement::new(2 + 5, Some(Transformation::Rotate270));
    cauldron
        .place(&materials, (1, 0), placement3, true, &mut scores)
        .unwrap();
    println!("scores after placement 3: {scores:?}");

    let placement4 = Placement::new(1 + 5 * 2, None);
    cauldron
        .place(&materials, (0, 1), placement4, true, &mut scores)
        .unwrap();
    println!("scores after placement 4: {scores:?}");

    let coverage = cauldron.calculate_coverage(&materials);
    println!("Coverage: {coverage:?}");
    assert_eq!(coverage.get_color_ratio(Color::Red, &cauldron), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Blue, &cauldron), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Green, &cauldron), 0.32);
    assert_eq!(coverage.get_color_ratio(Color::Yellow, &cauldron), 0.16);
    assert_eq!(coverage.get_color_ratio(Color::White, &cauldron), 0.);

    let final_scores = cauldron.calculate_final_score(&materials, &scores);

    assert_eq!(final_scores.as_slice(), vec![72, 50, 69]);
}

#[test]
fn test_overlap() {
    let straight_material =
        Material::new(Color::White, 15, Shape::from_binary([0b100, 0b100, 0b000]));
    let material_idx = (0, 0);
    let materials = vec![vec![straight_material]];

    let mut cauldron = Cauldron {
        size: 4,
        tiles: tiles![
            W 0, W 0, W 0, W 0,
            W 0, W 0, W 0, W 0,
            W 0, W 0, W 0, W 0,
            W 0, W 0, W 0, W 0,
        ],
        bonus_scores: (3, 5, 7),
        color: Color::White,
        properties: CauldronProperties::empty(),
    };

    let mut scores = vec![ColorScoreSet::default(); materials.len()];

    // placement at (1,1) downwards
    let placement_1 = Placement::new(1 + 4, None);
    // placement at (1,2) to the right
    let placement_2 = Placement::new(1 + 2 * 4, Some(Transformation::Rotate90));

    cauldron
        .place(&materials, material_idx, placement_1, true, &mut scores)
        .unwrap();

    assert_eq!(cauldron.get_tile((0, 2)).unwrap().level, 1);
    assert_eq!(cauldron.get_tile((2, 2)).unwrap().level, 1);

    // overwrite placement
    cauldron
        .place(&materials, material_idx, placement_2, true, &mut scores)
        .unwrap();

    assert_eq!(cauldron.get_tile((0, 2)).unwrap().level, 2);

    // redo first placement, which should clear up the previous one.
    // the tile that was originally occupied by that placement should have level 0 again

    cauldron
        .place(&materials, material_idx, placement_1, true, &mut scores)
        .unwrap();

    assert_eq!(cauldron.get_tile((0, 2)).unwrap().level, 3);
    assert_eq!(cauldron.get_tile((2, 2)).unwrap().level, 0);
}
