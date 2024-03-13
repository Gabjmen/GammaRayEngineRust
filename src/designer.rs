use egui::{Align2, Context};

pub static mut IS_THE_UI_HOVERED: bool = false;

pub fn gui(ui: &Context) {
    egui::Window::new("Streamline CFD")
        //.vscroll(true)
        .default_open(true)
        .max_width(1920.0)
        .max_height(1080.0)
        .default_width(800.0)
        .resizable(true)
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .show(&ui, |ui| {
            if ui.add(egui::Button::new("Click me")).clicked() {
                println!("PRESSED")
            }

            ui.label("Slider");

            ui.end_row();

            check_if_hovered(ui.ctx().is_pointer_over_area());
        });
}

pub fn check_if_hovered(is_hovered: bool) {
    unsafe { IS_THE_UI_HOVERED = is_hovered };
}