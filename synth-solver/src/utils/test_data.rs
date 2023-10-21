pub mod cauldron {
    use crate::*;
    pub fn uni_bag_5x5_bonus1() -> Cauldron {
        Cauldron {
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
        }
    }
}

pub mod goals {
    use crate::*;
    pub fn uni_bag() -> [Goal; 3] {
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
}

pub mod material {
    use crate::*;

    pub fn uni() -> Material {
        Material {
            color: Color::Yellow,
            effect_value: 15,
            shape: Shape::from_binary([0b100, 0b100, 0b100]),
        }
    }

    pub fn beehive() -> Material {
        Material {
            color: Color::Yellow,
            effect_value: 10,
            shape: Shape::from_binary([0b100, 0b110, 0b000]),
        }
    }

    pub fn broken_stone() -> Material {
        Material {
            color: Color::White,
            effect_value: 15,
            shape: Shape::from_binary([0b100, 0b100, 0b100]),
        }
    }
}
