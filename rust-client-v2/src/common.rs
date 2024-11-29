use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Setup,
    Auth,
    Lobby,
    Play,
}

pub fn cartesian_to_isometric(cart_x: f32, cart_y: f32) -> Vec2 {
    Vec2::new(
        cart_x - cart_y,         // X-axis in isometric space
        (cart_x + cart_y) / 2.0, // Y-axis in isometric space
    )
}

pub fn isometric_to_cartesian(iso_x: f32, iso_y: f32) -> Vec2 {
    Vec2::new(
        (2.0 * iso_y + iso_x) / 2.0, // Cartesian X
        (2.0 * iso_y - iso_x) / 2.0, // Cartesian Y
    )
}

pub enum RenderMode {
    Iso,
    Cart,
}

#[derive(Resource)]
pub struct RenderConfig {
    pub mode: RenderMode,
}

#[derive(Component)]
pub struct PhysicsParent(pub Entity);

#[derive(Component)]
pub struct RenderChild(pub Entity);

pub fn spawn_physics_render_pair<P, R>(
    commands: &mut Commands,
    physics_bundle: P,
    render_bundle: R,
) -> (Entity, Entity)
where
    P: Bundle,
    R: Bundle,
{
    // Spawn the physics parent entity
    let parent_entity = commands.spawn(physics_bundle).id();

    // Spawn the render child entity
    let child_entity = commands
        .spawn((PhysicsParent(parent_entity), render_bundle))
        .id();

    // Add the `RenderChild` component to the parent entity
    commands
        .entity(parent_entity)
        .insert(RenderChild(child_entity));

    (parent_entity, child_entity)
}
