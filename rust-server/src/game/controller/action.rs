use godot::builtin::Vector2;

pub enum EGameEntityAction {
    UpdateLocationTarget(Vector2),
    InstantUpdateLocation(Vector2),
    ThrowProjectile(Vector2, Vector2),
    ThrowFrozenOrb(Vector2, Vector2),
    ToggleHidden,
    HealthFullHeal,
}
