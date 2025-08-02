use std::{collections::HashMap, sync::LazyLock};

use macroquad::math::{Vec2, vec2};

pub static ARSTAVIKEN_DATA: LazyLock<HashMap<usize, Vec<Vec2>>> = LazyLock::new(|| {
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
