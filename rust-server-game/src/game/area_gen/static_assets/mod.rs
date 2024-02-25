use rust_common::proto::Orientation;
use rust_common::proto::StaticAssetType;

#[derive(Clone)]
pub struct GenStaticAssetType {
    pub asset_type: StaticAssetType,
    pub shape: Vec<(f32, f32)>,
    pub orientation: Orientation,
}
#[derive(Clone)]
pub struct GenStaticAsset {
    pub asset_type: GenStaticAssetType,
    pub coordinate: (i32, i32),
    pub layer: i32,
}
