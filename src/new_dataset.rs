use egui::{Context, Ui};

#[cfg_attr(feature="serde", derive(serde::Deserialize, serde::Serialize))]
pub struct NewDatasetWindow {
    enabled: bool,
    visible: bool,
}

impl Default for NewDatasetWindow {
    fn default() -> Self {
        Self {
            enabled: true,
            visible: true
        }
    }
}

impl super::Window for NewDatasetWindow {
    fn name(&self) -> &'static str {
        "New Dataset :]"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable([true, false])
            .default_width(280.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for NewDatasetWindow {
    fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Hello world!");
    }
}