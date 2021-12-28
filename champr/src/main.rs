pub mod champr;

use champr::ChampR;
use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2, Visuals},
    epi::App,
    run_native, NativeOptions,
};

impl App for ChampR {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        ctx.request_repaint();

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        CentralPanel::default().show(ctx, |ui| {
            self.render_top(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_list(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "ChampR"
    }
}

fn main() {
    let app = ChampR::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(360., 640.));
    run_native(Box::new(app), win_option);
}
