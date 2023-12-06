use godot::builtin::Vector2;

pub fn collide_rect_to_rect(
    r1_size: &Vector2,
    r1_pos: &Vector2,
    r2_size: &Vector2,
    r2_pos: &Vector2,
) -> bool {
    let r1x = r1_pos.x - r1_size.x / 2.0;
    let r1y = r1_pos.y - r1_size.y / 2.0;
    let r1w = r1_size.x;
    let r1h = r1_size.y;
    let r2x = r2_pos.x - r2_size.x / 2.0;
    let r2y = r2_pos.y - r2_size.y / 2.0;
    let r2w = r2_size.x;
    let r2h = r2_size.y;

    // https://www.jeffreythompson.org/collision-detection/rect-rect.php
    r1x + r1w >= r2x && r1x <= r2x + r2w && r1y + r1h >= r2y && r1y <= r2y + r2h
}
