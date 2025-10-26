use macroquad::math::{Vec2, vec2};
use std::{collections::HashMap, sync::LazyLock};

pub static ARSTAVIKEN_DATA: LazyLock<HashMap<usize, Vec<Vec2>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        1,
        vec![
            vec2(951.0438, 388.6979),
            vec2(2247.4133, 1074.7715),
            vec2(1503.0088, 914.7987),
            vec2(653.0921, 764.4674),
            vec2(415.0, 737.0),
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

pub fn get_obstacle_to_count() -> HashMap<usize, usize> {
    let mut obstacle_to_count = HashMap::new();
    for (id, vec) in ARSTAVIKEN_DATA.iter() {
        obstacle_to_count.insert(*id, vec.len());
    }

    obstacle_to_count
}
