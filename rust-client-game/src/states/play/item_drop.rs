use bevy::{prelude::*, utils::HashSet};
use bevy_prototype_lyon::prelude::*;

use rust_common_game::prelude::*;

use crate::{
    common::{cartesian_to_isometric_vec2, AppState, Z_ITEM_DROPPED_NAME_PLATE},
    states::play::PlaySceneTag,
};

use super::cursor::{HoverableEntity, HoverableEntityKind};

#[derive(Component)]
struct ItemDroppedRender {
    pub stroke_color: Color,
}

fn item_dropped_style(item_dropped: &ItemDropped) -> (Vec2, Color, Color, f32) {
    let default_size = Vec2::new(64., 14.);
    match item_dropped.ratity {
        ItemRarity::Common => (
            default_size * 0.85,
            Color::srgb_u8(0, 0, 0),
            Color::srgb_u8(146, 138, 108),
            1.,
        ),
        ItemRarity::Magic => (
            default_size * 0.85,
            Color::srgb_u8(4, 20, 38),
            Color::srgb_u8(29, 74, 241),
            1.,
        ),
        ItemRarity::Rare => (
            default_size * 0.95,
            Color::srgb_u8(235, 155, 61),
            Color::srgb_u8(0, 0, 0),
            1.,
        ),
        ItemRarity::Unique => (
            default_size * 1.,
            Color::srgb_u8(255, 255, 255),
            Color::srgb_u8(234, 51, 35),
            2.,
        ),
    }
}

fn on_new_item_dropped(
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut commands: Commands,
    q: Query<(Entity, &ItemDropped), Added<ItemDropped>>,
) {
    let mut audio_played = HashSet::default();

    for (entity, item_dropped) in &q {
        let (size, fill_color, stroke_color, stroke_line_width) = item_dropped_style(item_dropped);
        let shape = shapes::Rectangle {
            extents: size,
            ..default()
        };

        let position_iso = cartesian_to_isometric_vec2(&item_dropped.position);
        let position_iso_z =
            Z_ITEM_DROPPED_NAME_PLATE + (1. - ((position_iso.y) / map.map_px_size.y));
        commands.entity(entity).insert((
            ItemDroppedRender { stroke_color },
            HoverableEntity::new(HoverableEntityKind::DroppedItem, size),
            PlaySceneTag,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(position_iso.extend(position_iso_z)),
                ..default()
            },
            Stroke::new(stroke_color, stroke_line_width),
            Fill::color(fill_color),
        ));

        if let Some(sound_effect) = &item_dropped.sound_effect() {
            // Prevent to play the same sound multiple times in a single frame
            if audio_played.insert(*sound_effect) {
                let audio_source_alert_2 =
                    asset_server.load::<AudioSource>(sound_effect.audio_path());
                commands.spawn((AudioPlayer(audio_source_alert_2), PlaybackSettings::DESPAWN));
            }
        }
    }
}

fn update_item_dropped_hover_state(
    mut query: Query<(&ItemDroppedRender, &HoverableEntity, &mut Stroke), Changed<HoverableEntity>>,
) {
    for (item_dropped_render, hoverable_entity, mut stroke) in query.iter_mut() {
        if hoverable_entity.is_hovered() {
            stroke.color = Color::linear_rgb(0., 1., 0.)
        } else {
            stroke.color = item_dropped_render.stroke_color
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
