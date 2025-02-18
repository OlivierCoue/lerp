use bevy::prelude::*;

use crate::common::AppState;

#[derive(Clone, Copy)]
pub enum HoverableEntityKind {
    DroppedItem,
}

#[derive(Component, Clone, Copy)]
pub struct HoverableEntity {
    kind: HoverableEntityKind,
    size: Vec2,
    is_hover: bool,
}
impl HoverableEntity {
    pub fn new(kind: HoverableEntityKind, size: Vec2) -> Self {
        Self {
            kind,
            size,
            is_hover: false,
        }
    }
    pub fn is_hovered(&self) -> bool {
        self.is_hover
    }
}

#[derive(Component, Clone, Copy)]
struct HoveredEntity {
    kind: HoverableEntityKind,
    entity: Entity,
}
impl HoveredEntity {
    pub fn new(kind: HoverableEntityKind, entity: Entity) -> Self {
        Self { kind, entity }
    }
}

#[derive(Resource)]
pub struct CursorState {
    entity_hover: Option<HoveredEntity>,
}

fn detect_entity_hover(
    windows: Query<&Window>,
    mut cursor_state: ResMut<CursorState>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(Entity, &Transform, &mut HoverableEntity)>,
) {
    let (camera, cam_transform) = camera_q.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(cam_transform, cursor_position) {
            let mut new_entity_hover = None;

            for (entity, transform, hoverable_entity) in query.iter() {
                let half_size = hoverable_entity.size / 2.0;
                let entity_pos = transform.translation.truncate();

                // AABB collision check
                if world_position.x >= entity_pos.x - half_size.x
                    && world_position.x <= entity_pos.x + half_size.x
                    && world_position.y >= entity_pos.y - half_size.y
                    && world_position.y <= entity_pos.y + half_size.y
                {
                    new_entity_hover = Some((entity, *hoverable_entity));
                    break;
                }
            }

            // If a new entity is hovered, update state
            if let Some((new_entity, new_hoverable)) = &new_entity_hover {
                let mut same_as_previous = false;

                // Check if an entity was already hovered and if its the same as the new one
                if let Some(previous_hovered) = &cursor_state.entity_hover {
                    if previous_hovered.entity != *new_entity {
                        // Set previous entity to is_hover = false if it still exist
                        if let Ok((_, _, mut previous_hoverable)) =
                            query.get_mut(previous_hovered.entity)
                        {
                            previous_hoverable.is_hover = false;
                        }
                    } else {
                        same_as_previous = true;
                    }
                }

                if !same_as_previous {
                    // Set new entity to is_hover = true and save it in CursorState
                    // It is safe to unwrap here as new_entity_hover.entity come the query
                    let (_, _, mut new_hoverable_entity) = query.get_mut(*new_entity).unwrap();
                    new_hoverable_entity.is_hover = true;
                    cursor_state.entity_hover =
                        Some(HoveredEntity::new(new_hoverable.kind, *new_entity));
                }

            // Set CursorState to None with .take() and set previous entity to is_hover = false if it still exist
            } else if let Some(previous_hovered) = cursor_state.entity_hover.take() {
                if let Ok((_, _, mut previous_hoverable)) = query.get_mut(previous_hovered.entity) {
                    previous_hoverable.is_hover = false;
                }
                if cursor_state.entity_hover.is_some() {
                    cursor_state.entity_hover = None;
                }
            }
        }
    }
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState { entity_hover: None });
        app.add_systems(
            Update,
            (detect_entity_hover).run_if(in_state(AppState::Play)),
        );
    }
}
