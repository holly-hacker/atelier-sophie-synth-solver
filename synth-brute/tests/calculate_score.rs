use synth_brute::*;

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

#[test]
fn test_calculation() {
    let items = vec![
        vec![Item::new(Color::Yellow, 15, Shape::from_binary([0b100, 0b100, 0b100])); 2],
        vec![Item::new(
            Color::Yellow,
            10,
            Shape::from_binary([0b100, 0b110, 0b000]),
        )],
        vec![Item::new(
            Color::White,
            15,
            Shape::from_binary([0b100, 0b100, 0b100]),
        )],
    ];

    // TODO: capabilities
    // - Grandma's Cauldron
    // - Bonus Display Level 1
    // - points: 3/5/7
    let mut playfield = Playfield {
        width: 5,
        data: tiles![
            B 0, G 0, Y 0, Y 0, W 0,
            W 0, Y 0, Y 0, Y 0, Y 1,
            R 0, Y 0, R 1, R 0, Y 0,
            R 0, Y 0, R 0, R 0, Y 1,
            W 0, Y 2, Y 0, Y 0, Y 0,
        ],
    };

    let mut scores = vec![ColorScoreSet::default(); items.len()];

    let placement1 = Placement::new(2 + 5, ());
    *scores[0].get_mut(items[0][0].color) += playfield.place(&items, (0, 0), placement1);

    let placement2 = Placement::new(1 + 5 * 3, ());
    *scores[1].get_mut(items[1][0].color) += playfield.place(&items, (1, 0), placement2);

    let placement3 = Placement::new(3 + 5 * 2, ());
    *scores[2].get_mut(items[2][0].color) += playfield.place(&items, (2, 0), placement3);

    let placement4 = Placement::new(0, ());
    *scores[0].get_mut(items[0][1].color) += playfield.place(&items, (0, 1), placement4);

    let coverage = playfield.calculate_coverage();
    assert_eq!(coverage.get_color_ratio(Color::Red, &playfield), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Blue, &playfield), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Green, &playfield), 0.);
    assert_eq!(coverage.get_color_ratio(Color::Yellow, &playfield), 0.36);
    assert_eq!(coverage.get_color_ratio(Color::White, &playfield), 0.12);

    let final_scores = scores
        .iter()
        .zip(items.iter())
        .map(|(score_set, item_group)| score_set.calculate_score(item_group, &coverage, &playfield))
        .collect::<Vec<_>>();

    assert_eq!(final_scores, vec![48, 39, 28]);
}
