use caption_manager_rust::AppState;

fn main() -> eframe::Result<()> {

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([400.0, 300.0])
        ,
        ..Default::default()
    };

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(caption_manager_rust::AppState::new(cc))),
    )
}
