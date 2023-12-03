use crate::utils::Coord;

pub enum EGameEntityAction {
    UpdateLocationTarget(Coord),
    InstantUpdateLocation(Coord),
    ThrowProjectile(Coord, Coord),
    ThrowFrozenOrb(Coord, Coord),
    ToggleHidden,
}
