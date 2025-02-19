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

fn on_new_item_dropped(
    mut commands: Commands,
    q: Query<(Entity, &ItemDropped), Added<ItemDropped>>,
) {
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
    }
}

fn update_item_dropped_hover_state(
    mut query: Query<
        (&HoverableEntity, &mut Stroke),
        (With<ItemDroppedRender>, Changed<HoverableEntity>),
    >,
) {
    for (hoverable_entity, mut stroke) in query.iter_mut() {
        if hoverable_entity.is_hovered() {
            stroke.color = Color::linear_rgb(0., 1., 0.)
        } else {
            stroke.color = Color::linear_rgb(1., 0., 0.)
        }
    }
}

pub struct ItemDropPlugin;

impl Plugin for ItemDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_new_item_dropped, update_item_dropped_hover_state).run_if(in_state(AppState::Play)),
        );
    }
}
