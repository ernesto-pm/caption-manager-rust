use crate::models::Dataset;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AppState {
    datasets: Vec<Dataset>,
    temp_new_dataset: Dataset,
    show_new_dataset_form: bool
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            datasets: Vec::new(),
            temp_new_dataset: Dataset::default(),
            show_new_dataset_form: false
        }
    }
}

impl AppState {
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
}

impl eframe::App for AppState {
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
                    if ui.button("New dataset").clicked() {
                        self.show_new_dataset_form = true;
                        ui.close_menu();
                    }
                });

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Dataset Manager");

            ui.vertical(|ui| {
                for dataset in &self.datasets {
                    ui.label(&dataset.name);
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.temp_new_dataset.name);
                ui.text_edit_singleline(&mut self.temp_new_dataset.directory_abs_path);

                if ui.button("add").clicked() {
                    self.datasets.push(self.temp_new_dataset.clone());
                }
            });

            if self.show_new_dataset_form {
                egui::Window::new("Add new dataset")
                    .open(&mut self.show_new_dataset_form)
                    .show(ctx, |ui| {
                        ui.heading("New dataset");
                    });
            }

        });
    }
}