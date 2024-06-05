use std::collections::BTreeSet;
use crate::models::Dataset;
use crate::Window;
use egui::{Context};
use crate::app_state::AppState;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    app_state: AppState,
    //datasets: Vec<Dataset>,
    //temp_new_dataset: Dataset,

    #[serde(skip)]
    windows: Vec<Box<dyn Window>>,
    open: BTreeSet<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut open = BTreeSet::new();

        Self {
            //datasets: Vec::new(),
            //temp_new_dataset: Dataset::default(),
            app_state: AppState::default(),
            open: open,
            windows: vec![
                Box::<super::new_dataset::NewDatasetWindow>::default()
            ],
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
         */

        Default::default()
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self {windows, open, .. } = self;
        for window in windows {
            let mut is_open = open.contains(window.name());
            window.show(ctx, &mut is_open, &mut self.app_state);
            set_open(open, window.name(), is_open);
        }
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // or we can destructure let Self {windows, .. } = self; instead, which rust assumes is going to mutate the self. struct
                    for window in &mut self.windows {
                        if ui.button(window.name()).clicked() {
                            set_open(&mut self.open, window.name(), true);
                            ui.close_menu();
                        }
                    }
                });

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Hello world");

            ui.vertical(|ui| {
                for dataset in &self.app_state.datasets {
                    ui.label(&dataset.name);
                }
            });
        });

        self.windows(ctx);
    }


}

// ----------------------------------------------------------------------------

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// ----------------------------------------------------------------------------