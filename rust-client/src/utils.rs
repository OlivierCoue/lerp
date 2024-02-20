use godot::{
    builtin::{Vector2, Vector2i},
    log::godot_print,
};
use rust_common::proto::TileType;

pub fn iso_to_cart(vect2: &Vector2) -> Vector2 {
    Vector2 {
        x: vect2.x * 0.5 + vect2.y * 1.0,
        y: vect2.x * -0.5 + vect2.y * 1.0,
    }
}

pub fn cart_to_iso(vect2: &Vector2) -> Vector2 {
    Vector2 {
        x: vect2.x * 1.0 + vect2.y * -1.0,
        y: vect2.x * 0.5 + vect2.y * 0.5,
    }
}

pub fn tile_type_to_atlas_coord(tile_type: &TileType) -> Vector2i {
    match tile_type {
        TileType::Floor => Vector2i::new(2, 0),
        TileType::Forest => Vector2i::new(0, 1),
        TileType::Rock => Vector2i::new(0, 1),
        TileType::Water => Vector2i::new(3, 0),
    }
}

pub enum Direction {
    E,
    SEE,
    SE,
    SSE,
    S,
    SSW,
    SW,
    SWW,
    W,
    NWW,
    NW,
    NNW,
    N,
    NNE,
    NE,
    NEE,
}

pub fn angle_to_direction(angle: f32) -> Direction {
    if (-11.25..11.25).contains(&angle) {
        return Direction::E;
    } else if (11.25..33.75).contains(&angle) {
        return Direction::SEE;
    } else if (33.75..56.25).contains(&angle) {
        return Direction::SE;
    } else if (56.25..78.75).contains(&angle) {
        return Direction::SSE;
    } else if (78.75..101.25).contains(&angle) {
        return Direction::S;
    } else if (101.25..123.75).contains(&angle) {
        return Direction::SSW;
    } else if (123.75..146.25).contains(&angle) {
        return Direction::SW;
    } else if (146.25..168.75).contains(&angle) {
        return Direction::SWW;
    } else if (168.75..181.0).contains(&angle) || (-180.0..-168.75).contains(&angle) {
        return Direction::W;
    } else if (-168.75..-146.25).contains(&angle) {
        return Direction::NWW;
    } else if (-146.25..-123.75).contains(&angle) {
        return Direction::NW;
    } else if (-123.75..-101.25).contains(&angle) {
        return Direction::NNW;
    } else if (-101.25..-78.75).contains(&angle) {
        return Direction::N;
    } else if (-78.75..-56.25).contains(&angle) {
        return Direction::NNE;
    } else if (-56.25..-33.75).contains(&angle) {
        return Direction::NE;
    } else if (-33.75..-11.25).contains(&angle) {
        return Direction::NEE;
    }

    godot_print!("Invalid angle: {}", angle);
    Direction::N
}

pub fn get_walk_animation_for_direction(direction: &Direction) -> String {
    match direction {
        Direction::NE => String::from("walk_e"),
        Direction::NEE => String::from("walk_see"),
        Direction::E => String::from("walk_se"),
        Direction::SEE => String::from("walk_sse"),
        Direction::SE => String::from("walk_s"),
        Direction::SSE => String::from("walk_ssw"),
        Direction::S => String::from("walk_sw"),
        Direction::SSW => String::from("walk_sww"),
        Direction::SW => String::from("walk_w"),
        Direction::SWW => String::from("walk_nww"),
        Direction::W => String::from("walk_nw"),
        Direction::NWW => String::from("walk_nnw"),
        Direction::NW => String::from("walk_n"),
        Direction::NNW => String::from("walk_nne"),
        Direction::N => String::from("walk_ne"),
        Direction::NNE => String::from("walk_nee"),
    }
}

pub fn get_idle_animation_for_direction(direction: &Direction) -> String {
    match direction {
        Direction::NE => String::from("idle_e"),
        Direction::NEE => String::from("idle_see"),
        Direction::E => String::from("idle_se"),
        Direction::SEE => String::from("idle_sse"),
        Direction::SE => String::from("idle_s"),
        Direction::SSE => String::from("idle_ssw"),
        Direction::S => String::from("idle_sw"),
        Direction::SSW => String::from("idle_sww"),
        Direction::SW => String::from("idle_w"),
        Direction::SWW => String::from("idle_nww"),
        Direction::W => String::from("idle_nw"),
        Direction::NWW => String::from("idle_nnw"),
        Direction::NW => String::from("idle_n"),
        Direction::NNW => String::from("idle_nne"),
        Direction::N => String::from("idle_ne"),
        Direction::NNE => String::from("idle_nee"),
    }
}

pub fn get_attack_animation_for_direction(direction: &Direction) -> String {
    match direction {
        Direction::NE => String::from("attack_e"),
        Direction::NEE => String::from("attack_see"),
        Direction::E => String::from("attack_se"),
        Direction::SEE => String::from("attack_sse"),
        Direction::SE => String::from("attack_s"),
        Direction::SSE => String::from("attack_ssw"),
        Direction::S => String::from("attack_sw"),
        Direction::SSW => String::from("attack_sww"),
        Direction::SW => String::from("attack_w"),
        Direction::SWW => String::from("attack_nww"),
        Direction::W => String::from("attack_nw"),
        Direction::NWW => String::from("attack_nnw"),
        Direction::NW => String::from("attack_n"),
        Direction::NNW => String::from("attack_nne"),
        Direction::N => String::from("attack_ne"),
        Direction::NNE => String::from("attack_nee"),
    }
}
