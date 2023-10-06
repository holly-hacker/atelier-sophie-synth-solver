#[macro_export]
macro_rules! tiles {
    (R $level:expr) => {tiles!(Red $level)};
    (B $level:expr) => {tiles!(Blue $level)};
    (G $level:expr) => {tiles!(Green $level)};
    (Y $level:expr) => {tiles!(Yellow $level)};
    (W $level:expr) => {tiles!(White $level)};

    ($color:ident $level:expr) => {
        Some(Tile {
            color: Color::$color,
            level: $level,
            played_color: None,
        })
    };

    ($($color:ident $level:expr,)*) => {
        tinyvec::array_vec![[Option<Tile>; 6 * 6] =>
            $(tiles!($color $level),)*
        ]
    };
}
