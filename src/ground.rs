use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
};
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, Vector2};

pub fn build_ground(width: u32, height: u32, cols: u32, rows: u32) -> Mesh {
    let middle_x = width as f32 / 2.0;
    let middle_z = height as f32 / 2.0;
    let tri_w = width as f32 / cols as f32;
    let tri_h = height as f32 / rows as f32;

    // vertices
    let vertices = (0..rows + 1)
        .flat_map(|row| {
            (0..cols + 1).map(move |col| {
                let x = col as f32 * tri_w;
                let z = row as f32 * tri_h;
                (x - middle_x, z - middle_z)
            })
        })
        .map(|(x, z)| [x, 0.0, z])
        .collect::<Vec<[f32; 3]>>();
    assert_eq!(
        vertices.len(),
        ((cols + 1) * (rows + 1)) as usize,
        "vertices.len()"
    );

    // indices
    let indices = (0..rows)
        .flat_map(|row| {
            (0..cols).flat_map(move |col| {
                vec![
                    (row + 1) * (cols + 1) + col + 1,
                    row * (cols + 1) + col + 1,
                    row * (cols + 1) + col,
                    (row + 1) * (cols + 1) + col,
                    (row + 1) * (cols + 1) + col + 1,
                    row * (cols + 1) + col,
                ]
            })
        })
        .collect::<Vec<u32>>();
    assert_eq!(indices.len(), (cols * rows * 6) as usize, "indices.len()");

    // normals and uvs
    let normals = vec![[0.0, 1.0, 0.0]; vertices.len()];
    let uvs = (0..rows + 1)
        .flat_map(|row| {
            (0..cols + 1).map(move |col| {
                let u = col as f32 / cols as f32;
                let v = row as f32 / rows as f32;
                (u, v)
            })
        })
        .map(|(u, v)| [u, v])
        .collect::<Vec<[f32; 2]>>();
    assert_eq!(uvs.len(), vertices.len(), "uvs.len()");

    // mesh
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn update_ground(mesh: &mut Mesh, t: f32, perm_table: &PermutationTable) {
    let vertices = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION);
    if let Some(vertices) = vertices {
        if let VertexAttributeValues::Float32x3(v) = vertices {
            for vertex in v.iter_mut() {
                let x = vertex[0];
                let z = vertex[2];
                let noise = perlin_2d(Vector2::new((x + t * 0.5) as f64, z as f64), perm_table);
                *vertex = [x, noise as f32 * 0.2, z];
            }
        }
    }
}
