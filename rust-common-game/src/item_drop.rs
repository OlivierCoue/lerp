use avian2d::prelude::Position;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget, NetworkIdentity};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Component)]
pub struct PendingItemDroppedPickup(pub Entity);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ItemDropped {
    pub position: Vec2,
}

fn pickup_item_dropped(
    identity: NetworkIdentity,
    mut commands: Commands,
    player_q: Query<
        (
            Entity,
            &Position,
            &PendingItemDroppedPickup,
            Option<&MovementTarget>,
        ),
        (Or<(With<Predicted>, With<ReplicationTarget>)>,),
    >,
    dropped_item_q: Query<&ItemDropped>,
) {
    for (player_entity, player_position, pending_item_dropped_pickup, player_movement_target) in
        player_q.iter()
    {
        if let Ok(item_dropped) = dropped_item_q.get(pending_item_dropped_pickup.0) {
            let distance_to = player_position.0.distance(item_dropped.position);

            // Pickup item if in radius
            if distance_to <= PLAYER_PICKUP_RADIUS {
                commands
                    .entity(player_entity)
                    .remove::<(PendingItemDroppedPickup, MovementTarget)>();
                if identity.is_server() {
                    commands.entity(pending_item_dropped_pickup.0).despawn();
                }
            // Set MovementTarget to item location if not already set
            } else if player_movement_target.is_none()
                || player_movement_target.is_some_and(|t| t.0 != item_dropped.position)
            {
                commands
                    .entity(player_entity)
                    .insert(MovementTarget(item_dropped.position));
            }
        } else {
            // Could not find the item, maybe already picked up, we remove the PendingItemDroppedPickup and potential MovementTarget
            commands
                .entity(player_entity)
                .remove::<(PendingItemDroppedPickup, MovementTarget)>();
        }
    }
}

fn cancel_pending_item_dropped_pickup(
    mut commands: Commands,
    mut player_cancel_action_ev: EventReader<PlayerCancelAction>,
    player_q: Query<
        Entity,
        (
            With<PendingItemDroppedPickup>,
            Or<(With<Predicted>, With<ReplicationTarget>)>,
        ),
    >,
) {
    for event in player_cancel_action_ev.read() {
        if let Ok(player_entity) = player_q.get(event.0) {
            commands
                .entity(player_entity)
                .remove::<(PendingItemDroppedPickup, MovementTarget)>();
        }
    }
}
pub struct ItemDropPlugin;

impl Plugin for ItemDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                pickup_item_dropped,
                cancel_pending_item_dropped_pickup.run_if(on_event::<PlayerCancelAction>),
            )
                .chain()
                .in_set(GameSimulationSet::ApplyPassiveEffects),
        );
    }
}
