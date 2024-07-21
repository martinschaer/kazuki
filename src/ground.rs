use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
};
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, Vector2};

pub fn build_ground(width: u32, height: u32, cols: u32, rows: u32) -> Mesh {
    let half_w = width as f32 / 2.0;
    let half_h = height as f32 / 2.0;
    let w = width as f32 / cols as f32;
    let h = height as f32 / rows as f32;

    // vertices
    let vertices = (0..rows)
        .flat_map(|row| {
            (0..cols)
                .map(move |col| {
                    let x = col as f32 * w;
                    let z = row as f32 * h;
                    let quad = if row % 2 == col % 2 {
                        [(0, 0), (1, 0), (1, 1), (0, 0), (1, 1), (0, 1)]
                    } else {
                        [(0, 0), (1, 0), (0, 1), (0, 1), (1, 0), (1, 1)]
                    };
                    quad.map(|(a, b)| (x + a as f32 * w, z + b as f32 * h))
                })
                .flatten()
        })
        .map(|(x, z)| [x - half_w, 0.0, z - half_h])
        .collect::<Vec<[f32; 3]>>();
    assert_eq!(vertices.len(), (cols * rows * 6) as usize, "vertices.len()");

    // indices
    let indices = (0..rows)
        .flat_map(|row| {
            (0..cols).flat_map(move |col| [0, 2, 1, 3, 5, 4].map(|x| row * cols * 6 + col * 6 + x))
        })
        .collect::<Vec<u32>>();
    assert_eq!(indices.len(), (cols * rows * 6) as usize, "indices.len()");

    // normals and uvs
    let normals = vec![[0.0, 1.0, 0.0]; vertices.len()];
    let uvs = (0..rows)
        .flat_map(|row| {
            (0..cols)
                .map(move |col| {
                    let u = col as f32 / cols as f32;
                    let v = row as f32 / rows as f32;
                    let coords = [(0, 0), (1, 0), (1, 1), (0, 0), (1, 1), (0, 1)];
                    coords.map(|(x, y)| (u + x as f32, v + y as f32))
                })
                .flatten()
        })
        .map(|(u, v)| [u, v])
        .collect::<Vec<[f32; 2]>>();
    assert_eq!(uvs.len(), vertices.len(), "uvs.len()");

    // vertex colors
    let colors = vec![[0., 0., 0., 0.]; vertices.len()];

    // mesh
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn update_ground(mesh: &mut Mesh, t: f32, perm_table: &PermutationTable) {
    let indices = mesh.indices().unwrap().clone();
    let vertices = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION);
    let mut normals = vec![];
    if let Some(vertices) = vertices {
        if let VertexAttributeValues::Float32x3(v) = vertices {
            for vertex in v.iter_mut() {
                let x = vertex[0];
                let z = vertex[2];
                let noise = perlin_2d(Vector2::new((x - t) as f64, z as f64), perm_table);
                vertex[1] = noise as f32 * 0.5;
            }
            if let Indices::U32(indices) = indices {
                normals = calculate_normals(v, &indices);
            }
        }
    }
    if let Some(VertexAttributeValues::Float32x3(n)) = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL) {
        *n = normals.iter().map(|x| [x.x, x.y, x.z]).collect();
    }
}

fn calculate_normals(vertices: &Vec<[f32; 3]>, indices: &Vec<u32>) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; vertices.len()];

    for chunk in indices.chunks(3) {
        let i0 = chunk[0] as usize;
        let i1 = chunk[1] as usize;
        let i2 = chunk[2] as usize;

        let v0: Vec3 = vertices[i0].into();
        let v1: Vec3 = vertices[i1].into();
        let v2: Vec3 = vertices[i2].into();

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        let normal = edge1.cross(edge2).normalize();

        normals[i0] += normal;
        normals[i1] += normal;
        normals[i2] += normal;
    }

    for normal in &mut normals {
        *normal = normal.normalize();
    }

    normals
}

#[cfg(test)]
mod tests {
    use bevy::render::mesh::Mesh;

    use super::build_ground;

    #[test]
    fn test_build_ground() {
        let cols = 2;
        let rows = 2;
        let vertex_count = (cols * rows * 6) as usize;
        let mesh = build_ground(cols, rows, 2, 2);
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len(),
            vertex_count
        );
        assert_eq!(mesh.indices().unwrap().len(), vertex_count);
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().len(),
            vertex_count
        );
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap().len(),
            vertex_count
        );
    }
}
