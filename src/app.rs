use egui::RichText;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.monospace("--INSERT--");
                });

                egui::Frame::none()
                    .fill(ui.style().visuals.panel_fill)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.monospace("Top");
                            ui.monospace("0,0");
                            ui.monospace("ci");
                            egui::warn_if_debug_build(ui);
                        });
                    });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::StripBuilder::new(ui)
                .size(egui_extras::Size::remainder().at_least(100.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            Sheet { rows: 100 }.ui(ui);
                        });
                    });
                });
        });
    }
}

struct Sheet {
    rows: usize,
}

impl Sheet {
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let row_height = egui::TextStyle::Monospace.resolve(ui.style()).size;

        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(30.0)) // row number
            .column(Column::initial(60.0)) // row number
            .column(Column::initial(60.0)) // row number
            .column(Column::initial(60.0)) // row number
            .column(Column::initial(60.0)) // row number
            .min_scrolled_height(0.0);

        table
            .header(20.0, |mut header| {
                header.col(|_| {});
                header.col(|ui| {
                    ui.label(RichText::new("A").monospace().strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("B").monospace().strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("C").monospace().strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("D").monospace().strong());
                });
            })
            .body(|body| {
                body.rows(row_height, self.rows, |row_index, mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new(row_index.to_string()).monospace().strong());
                    });
                    for col_name in ['A', 'B', 'C', 'D'] {
                        row.col(|ui| {
                            ui.monospace(format!("{col_name}{row_index}"));
                        });
                    }
                });
            });
    }
}
