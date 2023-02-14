#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use git2::{Repository, StatusOptions};

mod file;
mod colors;

use self::file::File;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "wpi fsae git helper",
        options,
        Box::new(|_cc| Box::new(GitHelper::default())),
    )
}

struct GitHelper {
    active_folder: String,
    repo: Option<Repository>
}

impl Default for GitHelper {
    fn default() -> Self {
        Self {
            active_folder: "".to_string(),
            repo: Some(Repository::open("").unwrap())
        }
    }
}

impl eframe::App for GitHelper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(colors::nord());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("wpi fsae git helper");
            ui.horizontal(|ui| {
                if ui.button("select folder").clicked() {
                    match tinyfiledialogs::select_folder_dialog("hi", &self.active_folder) {
                        Some(folder) => {
                            self.active_folder = folder;
                            match Repository::open(&self.active_folder) {
                                Ok(repo) => self.repo = Some(repo),
                                Err(e) => println!("error opening {} as git repo: {}", self.active_folder, e),
                            }
                        },
                        None => (),
                    }
                };
                ui.label(self.active_folder.clone())
            });
            match &self.repo {
                Some(repo) => {
                    for f in repo.statuses(Some(StatusOptions::new().include_untracked(true))).unwrap().iter() {
                        ui.add(File::new(f.path().unwrap()));
                    };
                },
                None => (),
            };
        });
    }
}