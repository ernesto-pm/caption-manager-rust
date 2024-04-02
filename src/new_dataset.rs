use egui::{Context, Ui};
use crate::app_state::AppState;
use crate::models::Dataset;

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

    fn show(&mut self, ctx: &Context, open: &mut bool, app_state: &mut AppState) {
        egui::Window::new(self.name())
            .open(open)
            .resizable([true, false])
            .default_width(280.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui, app_state);
            });
    }
}

impl super::View for NewDatasetWindow {
    fn ui(&mut self, ui: &mut Ui, app_state: &mut AppState) {
        ui.heading("Hello world!");

        if ui.button("Add dataset").clicked() {
            app_state.datasets.push(Dataset{
                name: "holi".to_owned(),
                directory_abs_path: "world".to_owned(),
            })
        }
    }
}