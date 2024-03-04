#![allow(non_snake_case)]

use eframe::{egui, NativeOptions, run_native};
use egui::{Button, CentralPanel, Context, Grid, Label, RichText, SidePanel, TextEdit, TopBottomPanel, Ui};

pub fn open_project_window() {
    let native_options = NativeOptions::default();
    let _ = run_native("GammaRayEngine", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
pub struct MyEguiApp {
    entry_title: String,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            entry_title: Default::default(),
        }
    }
}

fn my_button(ui: &mut Ui) {
    if ui.button("Browse..").clicked() {
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

                ui.horizontal(|ui| {
                    my_button(ui);
                });
            });
        });

        CentralPanel::default().show(ctx, |ui|{
            ui.add(TextEdit::singleline(&mut self.entry_title).hint_text("Search"))
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