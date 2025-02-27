use avian2d::prelude::Position;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicationTarget, NetworkIdentity};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ItemDroppedSound {
    Alter1,
    Alter2,
    Alter6,
}
impl ItemDroppedSound {
    pub fn audio_path(&self) -> &'static str {
        match &self {
            Self::Alter1 => "sound/item-drop/AlertSound1.mp3",
            Self::Alter2 => "sound/item-drop/AlertSound2.mp3",
            Self::Alter6 => "sound/item-drop/AlertSound6.mp3",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ItemRarity {
    Common,
    Magic,
    Rare,
    Unique,
}

#[derive(Component)]
pub struct PendingItemDroppedPickup(pub Entity);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ItemDropped {
    pub position: Vec2,
    pub ratity: ItemRarity,
}
impl ItemDropped {
    pub fn sound_effect(&self) -> Option<ItemDroppedSound> {
        match &self.ratity {
            ItemRarity::Common => None,
            ItemRarity::Magic => None,
            ItemRarity::Rare => Some(ItemDroppedSound::Alter2),
            ItemRarity::Unique => Some(ItemDroppedSound::Alter6),
        }
    }
}

#[derive(Event)]
pub struct ItemDroppedPickedUp(pub Entity);

fn pickup_item_dropped(
    identity: NetworkIdentity,
    mut commands: Commands,
    mut item_dropped_picked_up_ev: EventWriter<ItemDroppedPickedUp>,
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
                item_dropped_picked_up_ev.send(ItemDroppedPickedUp(pending_item_dropped_pickup.0));
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
        app.add_event::<ItemDroppedPickedUp>();
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
