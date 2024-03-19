use egui::{Align2, Context};

pub static mut IS_THE_UI_HOVERED: bool = false;
pub static mut FPS: f64 = 0.0;
pub static mut MIN_FPS: f32 = 0.0;

pub fn gui(ui: &Context) {
    egui::Window::new("Basic data")
        //.vscroll(true)
        .default_open(true)
        .default_width(200.0)
        .default_height(ui.screen_rect().height())
        .resizable(true)
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .show(&ui, |ui| {
            // if ui.add(egui::Button::new("Click me")).clicked() {
            //     println!("PRESSED")
            // }

            unsafe {
                ui.label(format!("FPS: {:.0}", FPS));
                ui.label(format!("MIN FPS: {:.0}", MIN_FPS));
            }

            ui.end_row();

            unsafe { IS_THE_UI_HOVERED = ui.ctx().is_pointer_over_area() }
        });
}
