use eframe::{run_native, epi::App, egui::CentralPanel, NativeOptions};

struct ChampR;

impl App for ChampR {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("something");
        });
    }

    fn name(&self) -> &str {
        "ChampR"
    }
}

fn main() {
    println!("hello");
    let app = ChampR;
    let win_option = NativeOptions::default();

    run_native(Box::new(app), win_option);
}