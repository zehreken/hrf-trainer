use std::{collections::HashMap, sync::LazyLock};

use macroquad::math::{Vec2, vec2};

pub static ARSTAVIKEN_DATA: LazyLock<HashMap<usize, Vec<Vec2>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        1,
        vec![
            vec2(951.0438, 388.6979),
            vec2(2247.4133, 1074.7715),
            vec2(1503.0088, 914.7987),
            vec2(653.0921, 764.4674),
        ],
    );
    // done
    map.insert(
        2,
        vec![
            vec2(2135.2144, 967.0786),
            vec2(2426.563, 1009.68115),
            vec2(578.11206, 642.18884),
            vec2(712.7273, 232.91821),
        ],
    );
    map.insert(3, vec![vec2(1106.206, 705.64716)]);
    map.insert(
        4,
        vec![vec2(1932.7383, 898.9731), vec2(2072.5542, 927.8682)],
    );

    map
});

pub static ARSTAVIKEN_DATA_SAT: LazyLock<HashMap<usize, Vec<Vec2>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        1,
        vec![
            vec2(4020.0, 2016.0),
            vec2(2660.0, 1709.0),
            vec2(1120.0, 1463.0),
            vec2(1647.0, 790.0),
        ],
    );
    map.insert(
        2,
        vec![
            vec2(4350.0, 1885.0),
            vec2(3803.0, 1786.0),
            vec2(971.0, 1219.0),
            vec2(1230.0, 464.0),
        ],
    );
    map.insert(3, vec![vec2(1933.0, 1309.0)]);
    map.insert(4, vec![vec2(3711.0, 1705.0), vec2(3477.0, 1661.0)]);

    map
});
