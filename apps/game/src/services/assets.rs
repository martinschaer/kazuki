use three_d::egui::{FontData, FontFamily};

pub fn load_fonts(gui_context: &three_d::egui::Context) {
    let mut fonts = three_d::egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Hack-Bold".to_owned(),
        FontData::from_static(include_bytes!("../../fonts/Hack-Bold.ttf")),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Hack-Bold".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("Hack-Bold".to_owned());
    gui_context.set_fonts(fonts);
}
