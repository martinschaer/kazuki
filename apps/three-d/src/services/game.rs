use crate::services::assets::load_fonts;
use three_d::egui::{Color32, FontFamily, RichText};
use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Kazuki".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut ignited = false;
    let mut fps = 0_u32;
    let mut next_fps_check = 0.0;
    let mut frames_count = 0_u32;
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

    let mut gui = three_d::GUI::new(&context);

    window.render_loop(move |mut frame_input| {
        if frame_input.accumulated_time > next_fps_check {
            next_fps_check += 500.0;
            fps = frames_count * 2;
            frames_count = 0;
        }
        frames_count += 1;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                if !ignited {
                    load_fonts(gui_context);
                    ignited = true;
                }
                three_d::egui::Area::new("text_area")
                    .fixed_pos(three_d::egui::pos2(32.0, 32.0))
                    .show(gui_context, |ui| {
                        ui.label(
                            RichText::new("Kazuki!")
                                .color(Color32::WHITE)
                                .size(32.0)
                                .family(FontFamily::Proportional),
                        );
                        ui.label(
                            RichText::new(format!("{} fps", fps))
                                .color(Color32::WHITE)
                                .size(10.0)
                                .family(FontFamily::Monospace),
                        );
                    });
            },
        );

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
            .write(|| gui.render())
            .render(&camera, &model, &[]);

        FrameOutput::default()
    })
}
