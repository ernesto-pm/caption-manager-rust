mod app;
mod models;

pub use app::AppState;

pub mod new_dataset;

// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Window {
    // Is the window enabled?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }
    // static pointer so we can use it as a key to our store's open/close state
    fn name(&self) -> &'static str;

    // triggers the show action
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}