use eframe::App;
use eframe::NativeOptions;
use nfd::Response;
use std::path::PathBuf;

mod logic;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let app = OrdinamentoApp {
        selected_directory: None,
        filter_text: String::from("Enter filter"),
        target_directory: String::from("Pick your folder name"),
    };

    eframe::run_native("Ordinamento", options, Box::new(move |_cc| Box::new(app)))
}

struct OrdinamentoApp {
    selected_directory: Option<PathBuf>,
    filter_text: String,
    target_directory: String,
}

impl App for OrdinamentoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Sorter");

            ui.horizontal(|ui| {
                ui.label("Filter by:");
                ui.text_edit_singleline(&mut self.filter_text);
            });

            ui.horizontal(|ui| {
                ui.label("Target Directory:");
                ui.text_edit_singleline(&mut self.target_directory);
            });

            ui.horizontal(|ui| {
                let select_dir_button = ui.button("Select Directory");
                select_dir_button
                    .clone()
                    .on_hover_text("Select a directory");

                if select_dir_button.clicked() {
                    if let Response::Okay(path) = nfd::open_pick_folder(None).unwrap() {
                        self.selected_directory = Some(PathBuf::from(path));
                    }
                }

                if let Some(selected_dir) = &self.selected_directory {
                    ui.label(format!("Selected Directory: {}", selected_dir.display()));
                } else {
                    ui.label("No directory selected.");
                }
            });

            ui.horizontal(|ui| {
                let run_button = ui.button("Run");
                run_button
                    .clone()
                    .on_hover_text("Run the file sorting logic");

                if run_button.clicked() {
                    if let Some(selected_dir) = &self.selected_directory {
                        logic::filter_and_move_files(
                            selected_dir,
                            &self.filter_text,
                            &self.target_directory,
                        );
                    }
                }
            });
        });
    }
}
