use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use rust_common_game::prelude::*;

use crate::{
    common::{cartesian_to_isometric_vec2, AppState, Z_DEBUG},
    states::play::PlaySceneTag,
};

use super::cursor::{HoverableEntity, HoverableEntityKind};

#[derive(Component)]
struct ItemDroppedRender;

fn on_loot(mut commands: Commands, q: Query<(Entity, &Loot), Added<Loot>>) {
    let mut i = 0;
    for (entity, loot) in &q {
        let size = Vec2::new(60., 12.);
        let shape = shapes::Rectangle {
            extents: size,
            ..default()
        };

        commands.entity(entity).insert((
            ItemDroppedRender,
            HoverableEntity::new(HoverableEntityKind::DroppedItem, size),
            PlaySceneTag,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(
                    cartesian_to_isometric_vec2(&loot.position).extend(Z_DEBUG),
                ),
                ..default()
            },
            Stroke::new(Color::linear_rgb(1., 0., 0.), 2.0),
            Fill::color(Color::linear_rgb(1., 1., 1.)),
        ));
        i += 1;
    }
    if i > 0 {
        println!("{}", i)
    }
}

fn update_loot_hover(
    mut query: Query<
        (&HoverableEntity, &mut Stroke),
        (With<ItemDroppedRender>, Changed<HoverableEntity>),
    >,
) {
    for (hoverable_entity, mut stroke) in query.iter_mut() {
        if hoverable_entity.is_hovered() {
            println!("Loot overed true");
            stroke.color = Color::linear_rgb(0., 1., 0.)
        } else {
            println!("Loot overed false");
            stroke.color = Color::linear_rgb(1., 0., 0.)
        }
    }
}

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_loot, update_loot_hover).run_if(in_state(AppState::Play)),
        );
    }
}
