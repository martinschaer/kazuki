#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Kazuki".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let viewport = window.viewport();
    let mut camera = Camera::new_orthographic(
        viewport,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        1.0,
        0.1,
        10.0,
    );

    let positions = vec![
        vec3(0.5, -0.5, 0.0),
        vec3(-0.5, -0.5, 0.0),
        vec3(0.0, 0.5, 0.0),
    ];
    let colors = vec![
        Color::new(255, 0, 0, 255),
        Color::new(0, 255, 0, 255),
        Color::new(0, 0, 255, 255),
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);
        model.animate(frame_input.accumulated_time as f32);
        for event in frame_input.events.iter() {
            match event {
                _ => {}
            }
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .render(&camera, &model, &[]);

        FrameOutput::default()
    })
}
