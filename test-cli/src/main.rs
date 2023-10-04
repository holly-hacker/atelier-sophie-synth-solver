use synth_brute::{errors::SynthError, *};

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

fn main() -> Result<(), SynthError> {
    println!("Hello, world!");
    let materials = get_input_materials();
    let cauldron = get_test_cauldron();
    let goals = get_uni_bag_goals();

    let time_before = std::time::Instant::now();
    let optimal_routes = find_optimal::find_optimal_routes(&cauldron, &materials, &goals);
    let elapsed = time_before.elapsed();
    println!(
        "Found {} optimal routes in {elapsed:?}",
        optimal_routes.len()
    );
    println!();

    for (result, route) in optimal_routes.iter() {
        let mut scores = vec![ColorScoreSet::default(); materials.len()];
        let mut cauldron = cauldron.clone();
        for (i, move_) in route.iter().enumerate() {
            let (placement_x, placement_y) = cauldron.get_position(move_.placement.index);
            println!(
                "[Move {}] Place material {}-{} at {placement_x}x{placement_y}",
                i + 1,
                move_.material_index.0,
                move_.material_index.1
            );
            cauldron.place(
                &materials,
                move_.material_index,
                move_.placement,
                &mut scores,
            )?;
            print_playfield_coverage(&cauldron);
            print_playfield(&cauldron);
            println!();
        }

        let coverage = cauldron.calculate_coverage();
        let final_score = scores
            .iter()
            .zip(materials.iter())
            .map(|(score_set, item_group)| {
                score_set.calculate_score(item_group, &coverage, &cauldron)
            })
            .collect::<Vec<_>>();
        println!("Result: {result:?} with score {final_score:?}");
    }

    Ok(())
}

fn print_playfield(playfield: &Cauldron) {
    use owo_colors::{OwoColorize, Style};

    for row in 0..playfield.size {
        for col in 0..playfield.size {
            let tile = playfield.get_tile((row, col));
            let Some(tile) = tile else {
                print!(" ");
                continue;
            };

            let mut style = Style::new();

            match tile.color {
                Color::Red => style = style.red(),
                Color::Blue => style = style.blue(),
                Color::Green => style = style.green(),
                Color::Yellow => style = style.yellow(),
                Color::White => style = style.white(),
            }

            match tile.played_color {
                Some(Color::Red) => style = style.on_truecolor(128, 32, 32),
                Some(Color::Blue) => style = style.on_truecolor(32, 32, 128),
                Some(Color::Green) => style = style.on_truecolor(32, 128, 32),
                Some(Color::Yellow) => style = style.on_truecolor(128, 128, 32),
                Some(Color::White) => style = style.on_truecolor(128, 128, 128),
                None => (),
            }

            print!("{}", tile.level.style(style));
        }
        println!();
    }
}

fn print_playfield_coverage(playfield: &Cauldron) {
    use owo_colors::OwoColorize;
    let coverage = playfield.calculate_coverage();
    println!(
        "Coverage: {} {} {} {} {}",
        format!(
            "{}%",
            100. * coverage.get_color_ratio(Color::Red, playfield)
        )
        .red(),
        format!(
            "{}%",
            100. * coverage.get_color_ratio(Color::Blue, playfield)
        )
        .blue(),
        format!(
            "{}%",
            100. * coverage.get_color_ratio(Color::Green, playfield)
        )
        .green(),
        format!(
            "{}%",
            100. * coverage.get_color_ratio(Color::Yellow, playfield)
        )
        .yellow(),
        format!(
            "{}%",
            100. * coverage.get_color_ratio(Color::White, playfield)
        )
        .white(),
    );
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
