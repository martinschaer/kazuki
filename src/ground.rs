use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

pub fn build_ground() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-0.5, 0.0, -0.5], [0.5, 0.0, -0.5], [-0.5, 0.0, 0.5]],
    )
}
