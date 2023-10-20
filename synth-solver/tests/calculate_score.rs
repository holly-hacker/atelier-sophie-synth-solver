use synth_solver::*;

#[test]
fn test_calculation() {
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

    // TODO: capabilities. currently using Grandma's Cauldron which gives 0/3/5/7
    let mut cauldron = Cauldron {
        size: 5,
        tiles: tiles![
            B 0, G 0, Y 0, Y 0, W 0,
            W 0, Y 0, Y 0, Y 0, Y 1,
            R 0, Y 0, R 1, R 0, Y 0,
            R 0, Y 0, R 0, R 0, Y 1,
            W 0, Y 2, Y 0, Y 0, Y 0,
        ],
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
