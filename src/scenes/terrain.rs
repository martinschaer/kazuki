use bevy::{
    prelude::*,
    render::mesh::{Mesh, VertexAttributeValues},
};
use bevy_rapier3d::geometry::Collider;
use kazuki::ground::update_ground;

use crate::data::{Cursor, Ground, GroundParams};

pub fn system_update_ground(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_query: Query<&Handle<Mesh>, With<Ground>>,
    res_ground_params: Res<GroundParams>,
) {
    let mesh_handle = mesh_query.get_single().expect("Query not successful");
    let mesh = meshes.get_mut(mesh_handle).unwrap();
    update_ground(
        mesh,
        time.elapsed_seconds(),
        &res_ground_params.permutation_table,
    );
}

pub fn system_update_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();
    let mut cursor = cursor_query.single_mut();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    // gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);

    // Update the cursor's position.
    cursor.translation = point;
}

pub fn system_update_cursor_collisions(
    mut meshes: ResMut<Assets<Mesh>>,
    cursor_query: Query<(&Collider, &Transform), With<Cursor>>,
    ground_query: Query<&Handle<Mesh>, With<Ground>>,
) {
    let (cursor_collider, transform) = cursor_query.single();

    let ground_handle = ground_query.single();
    let ground_mesh = meshes.get_mut(ground_handle).unwrap();
    let ground_vertices = ground_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

    let mut vertices_intersected = Vec::new();
    let mut vertices_not_intersected = Vec::new();

    if let VertexAttributeValues::Float32x3(v) = ground_vertices {
        for (i, vertex) in v.iter().enumerate() {
            if cursor_collider.contains_point(
                transform.translation,
                transform.rotation,
                Vec3::from(vertex.clone()),
            ) {
                vertices_intersected.push(i);
            } else {
                vertices_not_intersected.push(i);
            }
        }
    }

    let ground_vertices_color = ground_mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR).unwrap();
    if let VertexAttributeValues::Float32x4(v) = ground_vertices_color {
        for i in vertices_intersected {
            v[i] = [0.2, 0.2, 0.2, 0.2];
        }
        for i in vertices_not_intersected {
            v[i] = [1., 1., 1., 1.];
        }
    }
}
