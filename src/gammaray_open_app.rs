#![allow(non_snake_case)]

use std::{fs::File, result, path::Path, path::PathBuf};

use eframe::{egui, NativeOptions, run_native};
use egui::{Button, CentralPanel, Context, Grid, Label, RichText, SidePanel, TextEdit, TopBottomPanel, Ui};

use egui_file_dialog::FileDialog;

pub fn open_project_window() {
    let native_options = NativeOptions::default();
    let _ = run_native("GammaRayEngine", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
pub struct MyEguiApp {
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,
    entry_title: String,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            entry_title: Default::default(),
            file_dialog: FileDialog::new(),
            selected_file: None,
        }
    }
}

// pub fn browse_button(ui: &mut Ui) {
    
// }

pub fn open_button(ui: &mut Ui) {
    if ui.button("Open").clicked() {
        println!("Button clicked!");
    }
}

pub fn cancel_button(ui: &mut Ui) {
    if ui.button("Cancel").clicked() {
        println!("Button clicked!");
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("left_panel").show(ctx, |ui|{
            ui.label("2D Project");
            ui.label("3D Project")
        });

        TopBottomPanel::bottom("topBottomPanel").show(ctx, |ui| {
            Grid::new("id").show(ui, |ui|{
                ui.horizontal(|ui| {
                    ui.label("Project Location");
                    ui.add(TextEdit::singleline(&mut self.entry_title));
                    ui.label("Project Name");
                    ui.add(TextEdit::singleline(&mut self.entry_title));
                });
                ui.end_row();

                ui.horizontal(|ui| {
                    if ui.button("Browse..").clicked() {
                        self.file_dialog.select_directory();
                        println!("Button clicked!");
                    }
                    open_button(ui);
                    cancel_button(ui);
                });
                ui.end_row();
            });
        });

        CentralPanel::default().show(ctx, |ui|{
            ui.add(TextEdit::singleline(&mut self.entry_title).hint_text("Search"));
            CentralPanel::default().show(ctx, |ui| {
                if ui.button("Select file").clicked() {
                    // Open the file dialog to select a file.
                    self.file_dialog.select_file();
                }
    
                ui.label(format!("Selected file: {:?}", self.selected_file));
    
                // Update the dialog and check if the user selected a file
                if let Some(path) = self.file_dialog.update(ctx).selected() {
                    self.selected_file = Some(path.to_path_buf());
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "MyEguiApp",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MyEguiApp::new(cc)))
    )
}