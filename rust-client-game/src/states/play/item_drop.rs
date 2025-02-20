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
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    q: Query<(Entity, &ItemDropped), Added<ItemDropped>>,
) {
    let mut audio_played = false;

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

        // Prevent to play the same sound multiple times in a single frame
        if !audio_played {
            audio_played = true;
            let audio_source_alert_2 =
                asset_server.load::<AudioSource>("assets/sound/item-drop/AlertSound2.mp3");
            commands.spawn((AudioPlayer(audio_source_alert_2), PlaybackSettings::DESPAWN));
        }
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

fn on_item_dropped_picked_up(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut item_dropped_picked_up_ev: EventReader<ItemDroppedPickedUp>,
) {
    if item_dropped_picked_up_ev.read().next().is_some() {
        let audio_source = asset_server.load::<AudioSource>("assets/sound/item-pickup.mp3");
        commands.spawn((AudioPlayer(audio_source), PlaybackSettings::DESPAWN));
    }
}

pub struct ItemDropPlugin;

impl Plugin for ItemDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_new_item_dropped, update_item_dropped_hover_state).run_if(in_state(AppState::Play)),
        );
        app.add_systems(
            Update,
            (on_item_dropped_picked_up)
                .run_if(in_state(AppState::Play).and(on_event::<ItemDroppedPickedUp>)),
        );
    }
}
